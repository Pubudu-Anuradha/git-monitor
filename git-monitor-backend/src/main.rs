use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct GenericResponse {
    success: bool,
    message: String,
}

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let res = GenericResponse {
        success: true,
        message: "Hello World".to_string(),
    };
    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
