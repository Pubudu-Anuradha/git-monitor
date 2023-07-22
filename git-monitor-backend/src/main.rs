mod services;

use actix_web::{get, web::Json, App, HttpServer};
use services::repo::get_all_repos;
use services::user::get_username;

#[get("/")]
async fn hello() -> Json<String> {
    let username = get_username();
    Json("Hello ".to_string() + &username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_all_repos))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
