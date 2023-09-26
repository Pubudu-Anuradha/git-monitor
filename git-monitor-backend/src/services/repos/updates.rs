use std::path::Path;

use crate::{
  get_prisma_connection,
  prisma::{self, repo},
  services::user::get_home_dir,
};
use actix_web::web::Json;
use walkdir::WalkDir;

use super::utils;

pub async fn update_repos_from_device() -> Json<Vec<String>> {
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
    update_repo_from_device(repo).await;
  }
  Json(res)
}

pub async fn update_repo_from_device(repo: String) {
  let grepo = git2::Repository::open(Path::new(repo.as_str()));
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
    let state = utils::repository_state_to_string(grepo.state());
    let _res = match get_prisma_connection()
      .await
      .repo()
      .upsert(
        repo::UniqueWhereParam::DirEquals(dir.clone()),
        (
          dir.clone(),
          false,
          name.clone(),
          is_valid,
          state.clone(),
          vec![],
        ),
        vec![
          repo::SetParam::SetIsValid(is_valid),
          repo::SetParam::SetState(state),
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
            let (status, path) = utils::status_entry_to_string_pair(status);
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
