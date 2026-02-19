use actix_web::HttpResponse;
use std::process::Command;
use std::time::Instant;
use crate::models::LangResponse;

fn src_path() -> &'static str {
    if std::path::Path::new("assets/dafny/Verify.dfy").exists() {
        "assets/dafny/Verify.dfy"
    } else {
        "dafny/Verify.dfy"
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let dafny = std::env::var("DAFNY_PATH")
        .unwrap_or_else(|_| "dafny".to_owned());

    match Command::new(&dafny)
        .args(["verify", "--allow-warnings", src_path()])
        .output()
    {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            let result = if out.status.success() {
                format!("Dafny verification passed. {}", stdout)
            } else {
                format!("Dafny: {}", stdout)
            };
            HttpResponse::Ok().json(LangResponse::ok("Dafny", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("Dafny", format!("dafny not found: {e}"))),
    }
}
