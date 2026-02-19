use actix_web::HttpResponse;
use std::process::Command;
use std::time::Instant;
use crate::models::LangResponse;

fn src_path() -> &'static str {
    if std::path::Path::new("assets/fstar/Verify.fst").exists() {
        "assets/fstar/Verify.fst"
    } else {
        "fstar/Verify.fst"
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let fstar = std::env::var("FSTAR_PATH")
        .unwrap_or_else(|_| "fstar.exe".to_owned());

    match Command::new(&fstar)
        .args(["--admit_smt_queries", "true", src_path()])
        .output()
    {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_owned();
            let result = if out.status.success() {
                format!("F* verification passed. stdout={}", stdout)
            } else {
                format!("F* output: {} | {}", stdout, stderr)
            };
            HttpResponse::Ok().json(LangResponse::ok("F*", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("F*", format!("fstar.exe not found: {e}"))),
    }
}
