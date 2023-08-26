mod utils;

use super::super::prisma::{self, repo};
use super::user::get_home_dir;
use crate::get_prisma_connection;
use actix_web::web::{self, Json};
use git2::Repository;
use prisma_client_rust::QueryError;
use std::path::Path;
use walkdir::WalkDir;

pub fn repos() -> actix_web::Scope {
  web::scope("")
    .service(web::resource("").to(get_all))
    .service(web::resource("/update").to(update_repos))
    .service(
      web::resource("/repo")
        .route(web::get().to(get_repo))
        .route(web::post().to(set_repo)),
    )
}

async fn update_repos() -> Json<Vec<String>> {
  let home_dir = get_home_dir();
  let root = Path::new(home_dir.as_str());
  let res = Vec::from_iter(
    WalkDir::new(root)
      .follow_links(false)
      .into_iter()
      .filter(|file| {
        let file = file.as_ref().unwrap();
        file.file_type().is_dir()
          && file.file_name().eq_ignore_ascii_case(".git")
      })
      .map(|f| {
        f.unwrap()
          .path()
          .parent()
          .unwrap()
          .to_string_lossy()
          .to_string()
      }),
  );
  for repo in res.clone() {
    update_repo(repo).await;
  }
  Json(res)
}

async fn update_repo(repo: String) {
  let grepo = Repository::open(Path::new(repo.as_str()));
  if grepo.is_ok() {
    let grepo = grepo.unwrap();
    let name = repo[repo.rfind("/").unwrap_or(0) + 1..repo.len()].to_string();
    let dir = repo;
    let is_valid = true;
    let branches = grepo.branches(None);
    let mut status_options = git2::StatusOptions::new();
    status_options
      .include_unmodified(false)
      .include_untracked(true)
      .exclude_submodules(false)
      .sort_case_sensitively(false);
    let statuses = grepo.statuses(Some(&mut status_options));
    let state = utils::state_to_string(grepo.state());
    let _res = match get_prisma_connection()
      .await
      .repo()
      .upsert(
        prisma::repo::UniqueWhereParam::DirEquals(dir.clone()),
        (
          dir.clone(),
          false,
          name.clone(),
          is_valid,
          state.clone(),
          vec![],
        ),
        vec![
          prisma::repo::SetParam::SetIsValid(is_valid),
          prisma::repo::SetParam::SetState(state),
        ],
      )
      .exec()
      .await
    {
      Ok(data) => {
        let branches = branches.unwrap().filter(|b| b.is_ok()).map(|branch| {
          let (br, bt) = branch.unwrap();
          let name = br.name().unwrap().unwrap().to_string();
          let br_t = match bt {
            git2::BranchType::Local => "Local",
            git2::BranchType::Remote => "Remote",
          }
          .to_string();
          let is_head = br.is_head();
          let upstream = match br.upstream() {
            Ok(up) => up.name().unwrap().unwrap().to_string().clone(),
            Err(_) => "".to_string(),
          };
          (name, br_t, is_head, upstream)
        });
        let _res = get_prisma_connection()
          .await
          .branch()
          .delete_many(vec![prisma::branch::WhereParam::RepoDir(
            prisma::read_filters::StringFilter::Equals(dir.clone()),
          )])
          .exec()
          .await;
        println!("Deleted branches: {}", _res.unwrap_or(0));
        for (name, branch_type, is_head, upstream) in branches {
          let _res = get_prisma_connection()
            .await
            .branch()
            .upsert(
              prisma::branch::UniqueWhereParam::NameRepoDirEquals(
                data.name.clone(),
                data.dir.clone(),
              ),
              (
                name,
                branch_type,
                is_head,
                prisma::repo::UniqueWhereParam::DirEquals(data.dir.clone()),
                vec![prisma::branch::SetParam::SetUpstream(Some(upstream))],
              ),
              vec![],
            )
            .exec()
            .await;
        }
        if statuses.is_ok() {
          let cond = vec![prisma::status::WhereParam::RepoDir(
            prisma::read_filters::StringFilter::Equals(data.dir.clone()),
          )];
          let _res = get_prisma_connection()
            .await
            .status()
            .delete_many(Vec::from(cond))
            .exec()
            .await;
          println!("Deleted statuses: {}", _res.unwrap_or(0));
          for status in statuses.unwrap().into_iter() {
            let (status, path) = utils::status(status);
            let _res = get_prisma_connection()
              .await
              .status()
              .create(
                status,
                path,
                prisma::repo::UniqueWhereParam::DirEquals(data.dir.clone()),
                vec![],
              )
              .exec()
              .await;
          }
        }
      }
      Err(e) => match e {
        prisma_client_rust::QueryError::Execute(_er) => {
          println!("Error Executing: {}", _er.message());
        }
        prisma_client_rust::QueryError::Serialize(_er) => {
          println!("Error Serializing: {}", _er);
        }
        prisma_client_rust::QueryError::Deserialize(_er) => {
          println!("Error Deserializing: {}", _er);
        }
      },
    };
  }
}

async fn get_all(
) -> Json<Result<Vec<repo::Data>, prisma_client_rust::QueryError>> {
  println!("Getting all repos");
  let conn = get_prisma_connection().await;
  Json(
    conn
      .repo()
      .find_many(vec![])
      .with(prisma::repo::branches::fetch(vec![]))
      .with(prisma::repo::statuses::fetch(vec![]))
      .exec()
      .await,
  )
}

#[derive(serde::Deserialize)]
pub struct RepoRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
}

pub async fn get_repo(
  req: web::Json<RepoRequest>,
) -> Json<Result<Option<repo::Data>, prisma_client_rust::QueryError>> {
  let conn = get_prisma_connection().await;
  let path = req
    .path
    .as_str()
    .trim_end_matches(" ")
    .trim_end_matches("/");
  let abs_path = match req.absolute {
    true => path.to_string(),
    false => format!(
      "{}/{}",
      get_home_dir(),
      path
        .trim_start_matches(" ")
        .trim_start_matches("~")
        .trim_start_matches("/")
    ),
  };
  Json(
    conn
      .repo()
      .find_unique(repo::UniqueWhereParam::DirEquals(abs_path))
      .with(prisma::repo::branches::fetch(vec![]))
      .with(prisma::repo::statuses::fetch(vec![]))
      .exec()
      .await,
  )
}

#[derive(serde::Deserialize)]
pub struct SetRepoRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
  #[allow(dead_code)]
  managed: bool,
}

async fn set_repo(
  req: web::Json<SetRepoRequest>,
) -> actix_web::web::Json<Result<std::option::Option<prisma::repo::Data>, QueryError>> {
  let path = req
    .path
    .as_str()
    .trim_end_matches(" ")
    .trim_end_matches("/");
  let abs_path = match req.absolute {
    true => path.to_string(),
    false => format!(
      "{}/{}",
      get_home_dir(),
      path
        .trim_start_matches(" ")
        .trim_start_matches("~")
        .trim_start_matches("/")
    ),
  };
  let conn = get_prisma_connection().await;
  let __ = update_repo(abs_path.clone()).await;
  let _ = conn
      .repo()
      .update(
        prisma::repo::UniqueWhereParam::DirEquals(abs_path.clone()),
        vec![prisma::repo::managed::set(req.managed)],
      )
      .exec()
      .await;
  Json(
    conn
      .repo()
      .find_unique(repo::UniqueWhereParam::DirEquals(abs_path))
      .with(prisma::repo::branches::fetch(vec![]))
      .with(prisma::repo::statuses::fetch(vec![]))
      .exec()
      .await,
  )
}
