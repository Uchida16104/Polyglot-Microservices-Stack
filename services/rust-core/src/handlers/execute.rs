use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::AppState;
use crate::models::job::{ExecuteJob, ExecuteRequest, JobResponse, JobStatus, get_language_config};
use crate::error::AppError;

const EXECUTION_TIMEOUT_SECS: u64 = 30;
const MAX_OUTPUT_BYTES: usize = 1024 * 1024;

pub async fn submit_execute_job(
    State(state): State<Arc<AppState>>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(req): Json<ExecuteRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let compile_job = sqlx::query!(
        r#"SELECT id, language, output, status FROM compile_jobs WHERE id = $1"#,
        req.compile_job_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Compile job not found".to_string()))?;

    if compile_job.status != "completed" {
        return Err(AppError::BadRequest(
            "Compile job has not completed successfully".to_string(),
        ));
    }

    let execute_job_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO execute_jobs (id, compile_job_id, user_id, stdin_data, status)
        VALUES ($1, $2, $3, $4, 'running')
        "#,
        execute_job_id,
        req.compile_job_id,
        user_id,
        req.stdin_data,
    )
    .execute(&state.db)
    .await?;

    let lang_config = get_language_config(&compile_job.language)
        .ok_or_else(|| AppError::BadRequest("Unsupported language".to_string()))?;

    let start = Instant::now();

    let result = run_in_sandbox(
        &compile_job.language,
        compile_job.output.as_deref().unwrap_or(""),
        lang_config.run_command,
        req.stdin_data.as_deref(),
    )
    .await;

    let duration_ms = start.elapsed().as_millis() as i32;

    match result {
        Ok((stdout, stderr, exit_code)) => {
            sqlx::query!(
                r#"
                UPDATE execute_jobs
                SET stdout_data = $1, stderr_data = $2, exit_code = $3,
                    duration_ms = $4, status = 'completed', completed_at = NOW()
                WHERE id = $5
                "#,
                &stdout[..stdout.len().min(MAX_OUTPUT_BYTES)],
                &stderr[..stderr.len().min(MAX_OUTPUT_BYTES)],
                exit_code,
                duration_ms,
                execute_job_id,
            )
            .execute(&state.db)
            .await?;

            Ok(Json(serde_json::json!({
                "job_id": execute_job_id,
                "status": "completed",
                "stdout": &stdout[..stdout.len().min(MAX_OUTPUT_BYTES)],
                "stderr": &stderr[..stderr.len().min(MAX_OUTPUT_BYTES)],
                "exit_code": exit_code,
                "duration_ms": duration_ms,
            })))
        }
        Err(e) => {
            sqlx::query!(
                r#"
                UPDATE execute_jobs
                SET status = 'failed', stderr_data = $1, duration_ms = $2, completed_at = NOW()
                WHERE id = $3
                "#,
                e.to_string(),
                duration_ms,
                execute_job_id,
            )
            .execute(&state.db)
            .await?;

            Err(AppError::Internal(e.to_string()))
        }
    }
}

async fn run_in_sandbox(
    language: &str,
    binary_path_or_source: &str,
    run_command: Option<&str>,
    stdin_data: Option<&str>,
) -> anyhow::Result<(String, String, i32)> {
    let mut cmd = if let Some(runner) = run_command {
        let mut c = Command::new(runner);
        c.arg(binary_path_or_source);
        c
    } else {
        let mut c = Command::new(binary_path_or_source);
        c
    };

    cmd.stdin(std::process::Stdio::piped())
       .stdout(std::process::Stdio::piped())
       .stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn()?;

    if let (Some(stdin_data), Some(mut stdin)) = (stdin_data, child.stdin.take()) {
        stdin.write_all(stdin_data.as_bytes()).await?;
    }

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(EXECUTION_TIMEOUT_SECS),
        child.wait_with_output(),
    )
    .await
    .map_err(|_| anyhow::anyhow!("Execution timed out after {}s", EXECUTION_TIMEOUT_SECS))??;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    Ok((stdout, stderr, exit_code))
}
