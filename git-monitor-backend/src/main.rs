mod responses;

use actix_web::{get, web, App, HttpServer, Responder, Result};
use git_monitor_backend::{
    get_repos, get_username,
    responses::repo::{Repo, Repos},
};
use responses::generic::GenericResponse;
use std::path::Path;

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let username = get_username();
    let res = GenericResponse {
        success: true,
        message: "Hello ".to_string() + &username,
    };
    Ok(web::Json(res))
}

#[get("/repos")]
async fn repos<'a>() -> Result<impl Responder> {
    let username = get_username();
    let homedir = "/home/".to_string() + username.as_str();
    let root = Path::new(homedir.as_str());
    let repos = get_repos(root).map(|dir_entry| {
        let dir = dir_entry
            .path()
            .to_string_lossy()
            .trim_end_matches("/.git")
            .to_string();
        // Trimming the prefix from the path
        let name = dir[dir.rfind("/").unwrap_or(0) + 1..dir.len()].to_string();
        Repo { name, dir }
    });
    let mut re: Vec<Repo> = Vec::new();
    re.extend(repos);
    let res = Repos { repos: re };
    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(repos))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
