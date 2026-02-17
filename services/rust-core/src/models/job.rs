use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CompileJob {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub language: String,
    pub source_code: String,
    pub compiler_flags: Option<String>,
    pub status: JobStatus,
    pub output: Option<String>,
    pub error_output: Option<String>,
    pub exit_code: Option<i32>,
    pub duration_ms: Option<i32>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExecuteJob {
    pub id: Uuid,
    pub compile_job_id: Uuid,
    pub user_id: Uuid,
    pub stdin_data: Option<String>,
    pub stdout_data: Option<String>,
    pub stderr_data: Option<String>,
    pub exit_code: Option<i32>,
    pub duration_ms: Option<i32>,
    pub status: JobStatus,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    #[sqlx(rename = "queued")]
    Queued,
    #[sqlx(rename = "running")]
    Running,
    #[sqlx(rename = "completed")]
    Completed,
    #[sqlx(rename = "failed")]
    Failed,
    #[sqlx(rename = "cancelled")]
    Cancelled,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Queued    => write!(f, "queued"),
            JobStatus::Running   => write!(f, "running"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Failed    => write!(f, "failed"),
            JobStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompileRequest {
    pub project_id: Uuid,
    pub language: String,
    pub source_code: String,
    pub compiler_flags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteRequest {
    pub compile_job_id: Uuid,
    pub stdin_data: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub job_id: Uuid,
    pub status: JobStatus,
    pub message: String,
}

pub struct SupportedLanguage {
    pub name: &'static str,
    pub compiler: &'static str,
    pub extension: &'static str,
    pub run_command: Option<&'static str>,
}

pub fn get_language_config(language: &str) -> Option<SupportedLanguage> {
    match language.to_lowercase().as_str() {
        "rust" => Some(SupportedLanguage {
            name: "rust", compiler: "rustc", extension: "rs", run_command: None,
        }),
        "c" => Some(SupportedLanguage {
            name: "c", compiler: "gcc", extension: "c", run_command: None,
        }),
        "cpp" | "c++" => Some(SupportedLanguage {
            name: "cpp", compiler: "g++", extension: "cpp", run_command: None,
        }),
        "python" | "python3" => Some(SupportedLanguage {
            name: "python", compiler: "python3", extension: "py", run_command: Some("python3"),
        }),
        "javascript" | "js" => Some(SupportedLanguage {
            name: "js", compiler: "node", extension: "js", run_command: Some("node"),
        }),
        "go" => Some(SupportedLanguage {
            name: "go", compiler: "go", extension: "go", run_command: None,
        }),
        "zig" => Some(SupportedLanguage {
            name: "zig", compiler: "zig", extension: "zig", run_command: Some("zig"),
        }),
        "mojo" => Some(SupportedLanguage {
            name: "mojo", compiler: "mojo", extension: "mojo", run_command: Some("mojo"),
        }),
        "dafny" => Some(SupportedLanguage {
            name: "dafny", compiler: "dafny", extension: "dfy", run_command: Some("dafny"),
        }),
        "fstar" | "f*" => Some(SupportedLanguage {
            name: "fstar", compiler: "fstar.exe", extension: "fst", run_command: Some("fstar.exe"),
        }),
        _ => None,
    }
}
