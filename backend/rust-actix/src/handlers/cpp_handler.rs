use actix_web::HttpResponse;
use std::process::Command;
use std::time::Instant;
use crate::models::LangResponse;

fn src_path() -> &'static str {
    if std::path::Path::new("assets/cpp/compute.cpp").exists() {
        "assets/cpp/compute.cpp"
    } else {
        "cpp/compute.cpp"
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let src = src_path();
    let bin = "/tmp/cpp_compute";

    let compile = Command::new("g++")
        .args(["-O2", "-o", bin, src])
        .output();

    match compile {
        Err(e) => {
            return HttpResponse::Ok()
                .json(LangResponse::err("C++", format!("g++ not found: {e}")));
        }
        Ok(out) if !out.status.success() => {
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_owned();
            return HttpResponse::Ok()
                .json(LangResponse::err("C++", format!("Compile error: {stderr}")));
        }
        _ => {}
    }

    match Command::new(bin).output() {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            HttpResponse::Ok().json(LangResponse::ok("C++", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("C++", format!("Run error: {e}"))),
    }
}
