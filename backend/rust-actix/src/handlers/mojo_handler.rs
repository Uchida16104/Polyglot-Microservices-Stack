use actix_web::HttpResponse;
use std::process::Command;
use std::time::Instant;
use crate::models::LangResponse;

fn src_path() -> &'static str {
    if std::path::Path::new("assets/mojo/compute.mojo").exists() {
        "assets/mojo/compute.mojo"
    } else {
        "mojo/compute.mojo"
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let mojo = std::env::var("MOJO_PATH")
        .unwrap_or_else(|_| "mojo".to_owned());

    match Command::new(&mojo).args(["run", src_path()]).output() {
        Ok(out) => {
            let result = if out.status.success() {
                String::from_utf8_lossy(&out.stdout).trim().to_owned()
            } else {
                format!(
                    "Mojo SDK required (Magic): {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                )
            };
            HttpResponse::Ok().json(LangResponse::ok("Mojo", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok().json(LangResponse::err(
            "Mojo",
            format!("mojo not found: {e}. Install via: curl -ssL https://magic.modular.com | bash"),
        )),
    }
}
