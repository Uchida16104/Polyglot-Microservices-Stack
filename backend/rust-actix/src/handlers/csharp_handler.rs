use actix_web::HttpResponse;
use std::process::Command;
use std::time::Instant;
use crate::models::LangResponse;

fn proj_path() -> &'static str {
    if std::path::Path::new("assets/csharp/Processor.csproj").exists() {
        "assets/csharp"
    } else {
        "csharp"
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let dotnet_path = std::env::var("DOTNET_PATH")
        .unwrap_or_else(|_| "/opt/render/project/dotnet/dotnet".to_owned());

    match Command::new(&dotnet_path)
        .args(["run", "--project", proj_path(), "-c", "Release"])
        .output()
    {
        Ok(out) => {
            let result = if out.status.success() {
                String::from_utf8_lossy(&out.stdout).trim().to_owned()
            } else {
                format!("stderr: {}", String::from_utf8_lossy(&out.stderr).trim())
            };
            HttpResponse::Ok().json(LangResponse::ok("C#", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("C#", format!("dotnet not available: {e}"))),
    }
}
