use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use crate::models::job::{CompileJob, CompileRequest, JobResponse, JobStatus, get_language_config};
use crate::error::AppError;

pub async fn submit_compile_job(
    State(state): State<Arc<AppState>>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(req): Json<CompileRequest>,
) -> Result<(StatusCode, Json<JobResponse>), AppError> {
    if get_language_config(&req.language).is_none() {
        return Err(AppError::BadRequest(format!("Unsupported language: {}", req.language)));
    }

    let job_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO compile_jobs (id, project_id, user_id, language, source_code, compiler_flags, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'queued')
        "#,
        job_id,
        req.project_id,
        user_id,
        req.language,
        req.source_code,
        req.compiler_flags,
    )
    .execute(&state.db)
    .await?;

    let mut conn = state.redis.get_async_connection().await?;
    redis::cmd("LPUSH")
        .arg("compile_queue")
        .arg(job_id.to_string())
        .query_async::<_, ()>(&mut conn)
        .await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(JobResponse {
            job_id,
            status: JobStatus::Queued,
            message: "Compile job queued successfully".to_string(),
        }),
    ))
}

pub async fn get_compile_status(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<CompileJob>, AppError> {
    let job = sqlx::query_as!(
        CompileJob,
        r#"
        SELECT id, project_id, user_id, language, source_code, compiler_flags,
               status AS "status: JobStatus",
               output, error_output, exit_code, duration_ms, retry_count,
               created_at, completed_at
        FROM compile_jobs
        WHERE id = $1
        "#,
        job_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Compile job not found".to_string()))?;

    Ok(Json(job))
}
