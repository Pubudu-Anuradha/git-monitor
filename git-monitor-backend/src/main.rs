mod services;

use actix_web::{
  web::{resource, scope},
  App, HttpResponse, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(scope("repos").service(services::repos::repos()))
      .service(scope("").service(resource("/").to(|| HttpResponse::Ok())))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
