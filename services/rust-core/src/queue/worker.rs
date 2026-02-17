use std::sync::Arc;
use std::time::Instant;
use std::io::Write;
use tokio::process::Command;
use uuid::Uuid;

use crate::AppState;
use crate::models::job::get_language_config;

const MAX_RETRIES: i32 = 3;
const COMPILE_TIMEOUT_SECS: u64 = 60;

pub async fn run_compile_worker(state: Arc<AppState>) {
    tracing::info!("Compile worker started");

    loop {
        let mut conn = match state.redis.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Worker redis connection failed: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        let result: redis::RedisResult<Option<(String, String)>> = redis::cmd("BRPOP")
            .arg("compile_queue")
            .arg(5u64)
            .query_async(&mut conn)
            .await;

        match result {
            Ok(Some((_, job_id_str))) => {
                if let Ok(job_id) = Uuid::parse_str(&job_id_str) {
                    let state_clone = state.clone();
                    tokio::spawn(async move {
                        if let Err(e) = process_compile_job(state_clone, job_id).await {
                            tracing::error!("Compile job {job_id} failed: {e}");
                        }
                    });
                }
            }
            Ok(None) => {}
            Err(e) => {
                tracing::error!("Worker queue error: {e}");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }
}

async fn process_compile_job(state: Arc<AppState>, job_id: Uuid) -> anyhow::Result<()> {
    let job = sqlx::query!(
        r#"SELECT id, language, source_code, compiler_flags, retry_count
           FROM compile_jobs WHERE id = $1 AND status = 'queued'"#,
        job_id
    )
    .fetch_optional(&state.db)
    .await?;

    let job = match job {
        Some(j) => j,
        None => return Ok(()),
    };

    sqlx::query!(
        "UPDATE compile_jobs SET status = 'running' WHERE id = $1",
        job_id
    )
    .execute(&state.db)
    .await?;

    let lang = get_language_config(&job.language)
        .ok_or_else(|| anyhow::anyhow!("Unsupported language: {}", job.language))?;

    let tmp_dir = std::env::temp_dir().join(job_id.to_string());
    tokio::fs::create_dir_all(&tmp_dir).await?;

    let source_file = tmp_dir.join(format!("main.{}", lang.extension));
    tokio::fs::write(&source_file, &job.source_code).await?;

    let start = Instant::now();

    let output_bin = tmp_dir.join("output");
    let flags: Vec<&str> = job.compiler_flags
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .collect();

    let compile_result = match job.language.as_str() {
        "python" | "javascript" => {
            Ok(tokio::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: vec![],
                stderr: vec![],
            })
        }
        _ => {
            let mut cmd = Command::new(lang.compiler);
            cmd.arg(source_file.to_str().unwrap())
               .arg("-o")
               .arg(output_bin.to_str().unwrap());
            for flag in &flags {
                cmd.arg(flag);
            }
            cmd.stdout(std::process::Stdio::piped())
               .stderr(std::process::Stdio::piped());

            let child = cmd.spawn()?;
            tokio::time::timeout(
                std::time::Duration::from_secs(COMPILE_TIMEOUT_SECS),
                child.wait_with_output(),
            )
            .await
            .map_err(|_| anyhow::anyhow!("Compilation timed out"))?
        }
    };

    let duration_ms = start.elapsed().as_millis() as i32;

    let _ = tokio::fs::remove_dir_all(&tmp_dir).await;

    match compile_result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(-1);
            let status = if exit_code == 0 { "completed" } else { "failed" };

            let artifact_path = if exit_code == 0 {
                Some(output_bin.to_string_lossy().to_string())
            } else {
                None
            };

            sqlx::query!(
                r#"
                UPDATE compile_jobs
                SET status = $1, output = $2, error_output = $3, exit_code = $4,
                    duration_ms = $5, completed_at = NOW()
                WHERE id = $6
                "#,
                status,
                artifact_path.as_deref().unwrap_or(&stdout),
                stderr,
                exit_code,
                duration_ms,
                job_id,
            )
            .execute(&state.db)
            .await?;
        }
        Err(e) => {
            if job.retry_count < MAX_RETRIES {
                sqlx::query!(
                    "UPDATE compile_jobs SET status = 'queued', retry_count = retry_count + 1 WHERE id = $1",
                    job_id
                )
                .execute(&state.db)
                .await?;

                let mut conn = state.redis.get_async_connection().await?;
                redis::cmd("LPUSH")
                    .arg("compile_queue")
                    .arg(job_id.to_string())
                    .query_async::<_, ()>(&mut conn)
                    .await?;
            } else {
                sqlx::query!(
                    r#"
                    UPDATE compile_jobs
                    SET status = 'failed', error_output = $1, duration_ms = $2, completed_at = NOW()
                    WHERE id = $3
                    "#,
                    e.to_string(),
                    duration_ms,
                    job_id,
                )
                .execute(&state.db)
                .await?;
            }
        }
    }

    Ok(())
}
