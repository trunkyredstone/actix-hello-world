use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u8)]
enum ErrorCode {
    NoError = 0,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    error_code: ErrorCode,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        message: "Hello world!".to_string(),
        error_code: ErrorCode::NoError,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .wrap(Logger::new("%a \"%r\" %s \"%{User-Agent}i\" %D"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
