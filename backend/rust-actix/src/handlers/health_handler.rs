use actix_web::HttpResponse;
use crate::models::HealthResponse;

pub async fn handle() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status:    "healthy".to_owned(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version:   env!("CARGO_PKG_VERSION").to_owned(),
        langs:     vec![
            "Rust".into(), "C++".into(), "C#".into(), "Python3/FastAPI".into(),
            "Zig".into(), "Mojo".into(), "F*".into(), "Dafny".into(),
        ],
    })
}
