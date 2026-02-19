use actix_web::{web, HttpResponse};
use std::time::Instant;
use std::process::Command;
use crate::config::AppState;
use crate::models::LangResponse;

fn run_cmd(bin: &str, args: &[&str]) -> String {
    match Command::new(bin).args(args).output() {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_owned()
        }
        Ok(out) => format!(
            "exit({}): {}",
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr).trim()
        ),
        Err(e) => format!("not found: {e}"),
    }
}

fn cpp_result() -> String {
    let src = if std::path::Path::new("assets/cpp/compute.cpp").exists() {
        "assets/cpp/compute.cpp"
    } else {
        "cpp/compute.cpp"
    };
    match Command::new("g++").args(["-O2", "-o", "/tmp/cpp_all", src]).output() {
        Ok(c) if c.status.success() => run_cmd("/tmp/cpp_all", &[]),
        Ok(c) => format!("compile: {}", String::from_utf8_lossy(&c.stderr).trim()),
        Err(e) => format!("g++ missing: {e}"),
    }
}

fn rust_result() -> String {
    fn fib(n: u64) -> u64 {
        if n <= 1 { return n; }
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 2..=n { let c = a+b; a = b; b = c; }
        b
    }
    let primes: Vec<u64> = (2u64..30).filter(|&n| (2..n).all(|i| n % i != 0)).collect();
    format!("Rust | fib(15)={} | primes<30: {:?}", fib(15), primes)
}

pub async fn handle(state: web::Data<AppState>) -> HttpResponse {
    let t = Instant::now();

    let zig = std::env::var("ZIG_PATH").unwrap_or_else(|_| "/usr/local/zig/zig".into());
    let zig_src = if std::path::Path::new("assets/zig/compute.zig").exists() {
        "assets/zig/compute.zig"
    } else {
        "zig/compute.zig"
    };

    let dotnet = std::env::var("DOTNET_PATH")
        .unwrap_or_else(|_| "/usr/local/dotnet/dotnet".into());
    let cs_proj = if std::path::Path::new("assets/csharp/Processor.csproj").exists() {
        "assets/csharp"
    } else {
        "csharp"
    };

    let python_result = {
        let url = format!("{}/compute", state.python_url);
        match state.http_client.get(&url).send().await {
            Ok(r) => match r.json::<serde_json::Value>().await {
                Ok(j) => j["result"].as_str().unwrap_or("ok").to_owned(),
                Err(e) => format!("json: {e}"),
            },
            Err(e) => format!("http: {e}"),
        }
    };

    let results = vec![
        LangResponse::ok("Rust",           rust_result(),                                0),
        LangResponse::ok("C++",            cpp_result(),                                 0),
        LangResponse::ok("C#",             run_cmd(&dotnet, &["run", "--project", cs_proj, "-c", "Release"]), 0),
        LangResponse::ok("Python3/FastAPI",python_result,                                0),
        LangResponse::ok("Zig",            run_cmd(&zig, &["run", zig_src]),             0),
    ];

    HttpResponse::Ok().json(serde_json::json!({
        "all_results":       results,
        "total_duration_ms": t.elapsed().as_millis()
    }))
}
