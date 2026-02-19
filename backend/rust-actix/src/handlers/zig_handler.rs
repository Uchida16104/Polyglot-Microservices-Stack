use actix_web::HttpResponse;
use std::process::Command;
use std::time::Instant;
use crate::models::LangResponse;

fn src_path() -> &'static str {
    if std::path::Path::new("assets/zig/compute.zig").exists() {
        "assets/zig/compute.zig"
    } else {
        "zig/compute.zig"
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let zig = std::env::var("ZIG_PATH")
        .unwrap_or_else(|_| "/opt/render/project/zig/zig".to_owned());

    match Command::new(&zig).args(["run", src_path()]).output() {
        Ok(out) => {
            let result = if out.status.success() {
                String::from_utf8_lossy(&out.stdout).trim().to_owned()
            } else {
                format!(
                    "Zig error: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                )
            };
            HttpResponse::Ok().json(LangResponse::ok("Zig", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("Zig", format!("zig binary not found: {e}"))),
    }
}
