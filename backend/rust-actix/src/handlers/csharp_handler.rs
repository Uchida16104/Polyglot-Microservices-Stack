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

fn strip_dotnet_banner(raw: &str) -> String {
    // .NET CLI welcome message ends with a line composed entirely of dashes.
    // Everything after the last such separator line is the actual program output.
    let separator_idx = raw
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let t = line.trim();
            t.len() >= 10 && t.chars().all(|c| c == '-')
        })
        .map(|(i, _)| i)
        .last();

    match separator_idx {
        Some(idx) => raw
            .lines()
            .skip(idx + 1)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_owned(),
        None => raw.trim().to_owned(),
    }
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let dotnet_path = std::env::var("DOTNET_PATH")
        .unwrap_or_else(|_| "/opt/render/project/dotnet/dotnet".to_owned());

    match Command::new(&dotnet_path)
        .args(["run", "--project", proj_path(), "-c", "Release"])
        .env("DOTNET_NOLOGO",               "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_CLI_UI_LANGUAGE",      "en")
        .output()
    {
        Ok(out) => {
            let raw = if out.status.success() {
                String::from_utf8_lossy(&out.stdout).to_owned().into_owned()
            } else {
                format!("stderr: {}", String::from_utf8_lossy(&out.stderr).trim())
            };
            let result = strip_dotnet_banner(&raw);
            HttpResponse::Ok().json(LangResponse::ok("C#", result, t.elapsed().as_millis()))
        }
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("C#", format!("dotnet not available: {e}"))),
    }
}
