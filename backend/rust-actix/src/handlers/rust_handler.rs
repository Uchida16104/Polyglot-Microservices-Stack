use actix_web::HttpResponse;
use std::time::Instant;
use crate::models::LangResponse;

fn fib(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n { let c = a + b; a = b; b = c; }
    b
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    (2..=(n as f64).sqrt() as u64).all(|i| n % i != 0)
}

pub async fn handle() -> HttpResponse {
    let t = Instant::now();
    let primes: Vec<u64> = (2u64..50).filter(|&n| is_prime(n)).collect();
    let result = format!(
        "Rust | fib(15)={} | primes<50: {:?}",
        fib(15),
        primes
    );
    HttpResponse::Ok().json(LangResponse::ok("Rust", result, t.elapsed().as_millis()))
}
