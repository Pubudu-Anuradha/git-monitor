mod utils;

use crate::services::user::get_home_dir;
use actix_web::{
  web::{self, Json},
  HttpResponse,
};
use git2::{BranchType, Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

pub fn repos() -> actix_web::Scope {
  web::scope("")
    .service(web::resource("").to(|| HttpResponse::Ok()))
    .service(web::resource("/scanlocal").to(scan_local_repos))
    .service(web::resource("/repo").to(get_repo))
}

#[derive(Serialize)]
pub struct Repo {
  pub name: String,
  pub dir: String,
  pub is_valid: bool,
  pub state: String,
  pub statuses: Vec<String>,
  pub branches: Vec<Branch>,
}

#[derive(Serialize)]
pub struct Branch {
  name: String,
  branch_type: String,
  is_head: bool,
  upstream: String,
}

impl Repo {
  fn new(name: String, dir: String) -> Self {
    match Repository::open(Path::new(dir.as_str())) {
      Ok(repo) => {
        print!("");
        let mut options = StatusOptions::new();
        let branches = Vec::from_iter(
          repo
            .branches(None)
            .unwrap()
            .map(|b| Branch::new(b.unwrap())),
        );
        Self {
          name,
          dir,
          is_valid: true,
          state: utils::state_to_string(repo.state()),
          statuses: Vec::from_iter(
            repo
              .statuses(Some(&mut options))
              .unwrap()
              .into_iter()
              .map(|s| utils::status_to_string(s).to_string()),
          ),
          branches,
        }
      }
      Err(_) => Self {
        name,
        dir,
        is_valid: false,
        state: "invalid".to_string(),
        statuses: Vec::new(),
        branches: Vec::new(),
      },
    }
  }
}

impl Branch {
  fn new(b: (git2::Branch, BranchType)) -> Self {
    let branch = b.0;
    let upstream = match branch.upstream() {
      Ok(u) => u.name().unwrap().unwrap().to_string(),
      Err(_) => "".to_string(),
    };
    Self {
      name: branch.name().unwrap().unwrap().to_string(),
      branch_type: match b.1 {
        BranchType::Local => "Local",
        BranchType::Remote => "Remote",
      }
      .to_string(),
      is_head: branch.is_head(),
      upstream,
    }
  }
}

#[derive(Deserialize)]
pub struct RepoRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
}

pub async fn get_repo(req: web::Json<RepoRequest>) -> Json<Repo> {
  let path = req
    .path
    .as_str()
    .trim_end_matches(" ")
    .trim_end_matches("/");
  let name = path[path.rfind("/").unwrap_or(0) + 1..path.len()].to_string();

  Json(Repo::new(
    name,
    match req.absolute {
      true => path.to_string(),
      false => format!(
        "{}/{}",
        get_home_dir(),
        path
          .trim_start_matches(" ")
          .trim_start_matches("~")
          .trim_start_matches("/")
      ),
    },
  ))
}

pub async fn scan_local_repos() -> Json<Vec<Repo>> {
  let home_dir = get_home_dir();
  let root = Path::new(home_dir.as_str());
  let repos = Vec::from_iter(
    WalkDir::new(root)
      .follow_links(false)
      .into_iter()
      .filter(|f| {
        f.as_ref().is_ok_and(|d| {
          d.file_type().is_dir() && d.file_name().eq_ignore_ascii_case(".git")
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
        let name = dir[dir.rfind("/").unwrap_or(0) + 1..dir.len()].to_string();
        Repo::new(name, dir)
      }),
  );
  web::Json(repos)
}
