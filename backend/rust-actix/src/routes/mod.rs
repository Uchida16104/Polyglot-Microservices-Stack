use actix_web::web;
use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handlers::health_handler::handle))
        .service(
            web::scope("/api")
                .route("/rust",   web::get().to(handlers::rust_handler::handle))
                .route("/cpp",    web::get().to(handlers::cpp_handler::handle))
                .route("/csharp", web::get().to(handlers::csharp_handler::handle))
                .route("/python", web::get().to(handlers::python_handler::handle))
                .route("/zig",    web::get().to(handlers::zig_handler::handle))
                .route("/mojo",   web::get().to(handlers::mojo_handler::handle))
                .route("/fstar",  web::get().to(handlers::fstar_handler::handle))
                .route("/dafny",  web::get().to(handlers::dafny_handler::handle))
                .route("/all",    web::get().to(handlers::all_handler::handle)),
        );
}
