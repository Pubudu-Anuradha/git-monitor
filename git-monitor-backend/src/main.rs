mod responses;

use actix_web::{get, web, App, HttpServer, Responder, Result};
use users::{get_current_uid, get_user_by_uid};
use walkdir::WalkDir;

#[get("/")]
async fn hello() -> Result<impl Responder> {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_str().unwrap();
    let homedir = format!("/home/{}", username);

    let walker = WalkDir::new(homedir)
        .follow_links(false)
        .max_depth(10)
        .min_depth(1);

    for entry in walker {
        match entry {
            Ok(dir) => {
                if dir.file_type().is_dir() && dir.file_name().eq_ignore_ascii_case(".git") {
                    std::println!("{}", dir.path().to_string_lossy());
                }
            }
            Err(err) => {
                std::println!("{}", err.to_string());
            }
        }
    }

    let res = responses::generic::GenericResponse {
        success: true,
        message: "Hello ".to_string() + username,
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
