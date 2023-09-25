use actix_cors::Cors;
use actix_web::{
  middleware::Logger,
  web::{resource, scope},
  App, HttpResponse, HttpServer,
};
use git_monitor_backend::services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  HttpServer::new(|| {
    let cors_config = Cors::default()
      .allow_any_origin()
      .allow_any_method()
      .allow_any_header()
      .supports_credentials();
    App::new()
      .wrap(Logger::default())
      .wrap(cors_config)
      .service(services::repos::repo_scope())
      .service(scope("").service(resource("/").to(|| HttpResponse::Ok())))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
