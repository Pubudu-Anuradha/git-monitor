use actix_web::{
  get,
  web::{self, Json},
};
use git2::{Repository, RepositoryState, StatusEntry, StatusOptions};
use serde::Serialize;
use std::path::Path;
use walkdir::WalkDir;

use crate::services::user::get_home_dir;

#[derive(Serialize)]
pub struct Repo {
  pub name: String,
  pub dir: String,
  pub is_valid: bool,
  pub state: String,
  pub statuses: Vec<String>,
}

impl Repo {
  fn new(name: String, dir: String) -> Self {
    match Repository::open(Path::new(dir.as_str())) {
      Ok(repo) => {
        print!("");
        let mut options = StatusOptions::new();
        Self {
          name,
          dir,
          is_valid: true,
          state: state_to_string(repo.state()).to_string(),
          statuses: Vec::from_iter(
            repo
              .statuses(Some(&mut options))
              .unwrap()
              .into_iter()
              .map(|s| status_to_string(s).to_string()),
          ),
        }
      }
      Err(_) => Self {
        name,
        dir,
        is_valid: false,
        state: "invalid".to_string(),
        statuses: Vec::new(),
      },
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
    });
  let mut re: Vec<Repo> = Vec::new();
  re.extend(repos);
  web::Json(re)
}

fn state_to_string<'a>(state: RepositoryState) -> &'a str {
  match state {
    RepositoryState::Clean => "Clean",
    RepositoryState::Merge => "Merge",
    RepositoryState::Revert => "Revert",
    RepositoryState::RevertSequence => "RevertSequence",
    RepositoryState::CherryPick => "CherryPick",
    RepositoryState::CherryPickSequence => "CheeryPickSequence",
    RepositoryState::Bisect => "Bisect",
    RepositoryState::Rebase => "Rebase",
    RepositoryState::RebaseInteractive => "RebaseInteractive",
    RepositoryState::RebaseMerge => "RebaseMerge",
    RepositoryState::ApplyMailbox => "ApplyMailbox",
    RepositoryState::ApplyMailboxOrRebase => "ApplyMailboxOrRebase",
  }
}

fn status_to_string(entry: StatusEntry) -> String {
  let status = match entry.status() {
    git2::Status::CURRENT => "CURRENT",
    git2::Status::INDEX_NEW => "INDEX_NEW",
    git2::Status::INDEX_MODIFIED => "INDEX_MODIFIED",
    git2::Status::INDEX_DELETED => "INDEX_DELETED",
    git2::Status::INDEX_RENAMED => "INDEX_RENAMED",
    git2::Status::INDEX_TYPECHANGE => "INDEX_TYPECHANGE",
    git2::Status::WT_NEW => "WT_NEW",
    git2::Status::WT_MODIFIED => "WT_MODIFIED",
    git2::Status::WT_DELETED => "WT_DELETED",
    git2::Status::WT_TYPECHANGE => "WT_TYPECHANGE",
    git2::Status::WT_RENAMED => "WT_RENAMED",
    git2::Status::IGNORED => "IGNORED",
    git2::Status::CONFLICTED => "CONFLICTED",
    _ => "UNKNOWN",
  };
  let path = match entry.path() {
    Some(p) => p,
    None => "UNKNOWN",
  };

  format!("{} : {}", status, path)
}
