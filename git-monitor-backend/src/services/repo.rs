use actix_web::{
    get,
    web::{self, Json},
};
use git2::Repository;
use serde::Serialize;
use std::path::Path;
use walkdir::WalkDir;

use crate::services::user::get_home_dir;

#[derive(Serialize)]
pub struct Repo {
    pub name: String,
    pub dir: String,
    pub is_valid: bool,
}

impl Repo {
    fn new(name: String, dir: String) -> Self {
        let is_valid = match Repository::open(Path::new(dir.as_str())) {
            Ok(_) => true,
            Err(_) => false,
        };
        Self {
            name,
            dir,
            is_valid,
        }
    }
}

#[get("/repos")]
pub async fn get_all_repos<'a>() -> Json<Vec<Repo>> {
    let home_dir = get_home_dir();
    let root = Path::new(home_dir.as_str());
    let repos = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter(|f| {
            f.as_ref().is_ok_and(|d| {
                d.file_type().is_dir()
                    && d.file_name().eq_ignore_ascii_case(".git")
            })
        })
        .map(|f| f.unwrap())
        .map(|dir_entry| {
            let dir = dir_entry
                .path()
                .parent()
                .unwrap_or(root)
                .to_string_lossy()
                .to_string();
            // Trimming the prefix from the path
            let name =
                dir[dir.rfind("/").unwrap_or(0) + 1..dir.len()].to_string();
            Repo::new(name, dir)
        });
    let mut re: Vec<Repo> = Vec::new();
    re.extend(repos);
    web::Json(re)
}
