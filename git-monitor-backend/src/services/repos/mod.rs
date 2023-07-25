mod utils;

use super::super::prisma::{self, repo};
use super::user::get_home_dir;
use crate::get_prisma_connection;
use actix_web::web::{self, Json};
use git2::Repository;
use std::path::Path;
use walkdir::WalkDir;

pub fn repos() -> actix_web::Scope {
  web::scope("")
    .service(web::resource("").to(repos_handler))
    .service(web::resource("/update").to(update_repos))
  // .service(web::resource("").to(|| HttpResponse::Ok()))
  // .service(web::resource("/scanlocal").to(scan_local_repos))
  // .service(web::resource("/repo").to(get_repo))
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
          (dir.clone(), name.clone(), is_valid, state.clone(), vec![]),
          vec![
            prisma::repo::SetParam::SetIsValid(is_valid),
            prisma::repo::SetParam::SetState(state),
          ],
        )
        .exec()
        .await
      {
        Ok(data) => {
          let branches =
            branches.unwrap().filter(|b| b.is_ok()).map(|branch| {
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
  Json(res)
}

async fn repos_handler() -> Json<Vec<repo::Data>> {
  let _conn = get_prisma_connection().await;
  web::Json(Vec::new())
}

// #[derive(Serialize)]
// pub struct Repo {
//   pub name: String,
//   pub dir: String,
//   pub is_valid: bool,
//   pub state: String,
//   pub statuses: Vec<Status>,
//   pub branches: Vec<Branch>,
// }

// #[derive(Serialize)]
// pub struct Branch {
//   name: String,
//   branch_type: String,
//   is_head: bool,
//   upstream: String,
// }

// #[derive(Serialize)]
// pub struct Status {
//   pub status: String,
//   pub path: String,
// }

// impl Repo {
//   fn new(name: String, dir: String) -> Self {
//     match Repository::open(Path::new(dir.as_str())) {
//       Ok(repo) => {
//         print!("");
//         let mut options = StatusOptions::new();
//         options
//           .include_ignored(false)
//           .include_unmodified(false)
//           .include_untracked(true);
//         let branches = Vec::from_iter(
//           repo
//             .branches(None)
//             .unwrap()
//             .map(|b| Branch::new(b.unwrap())),
//         );
//         Self {
//           name,
//           dir,
//           is_valid: true,
//           state: utils::state_to_string(repo.state()),
//           statuses: Vec::from_iter(
//             repo
//               .statuses(Some(&mut options))
//               .unwrap()
//               .into_iter()
//               .map(|s| utils::status(s)),
//           ),
//           branches,
//         }
//       }
//       Err(_) => Self {
//         name,
//         dir,
//         is_valid: false,
//         state: "invalid".to_string(),
//         statuses: Vec::new(),
//         branches: Vec::new(),
//       },
//     }
//   }
// }

// impl Branch {
//   fn new(b: (git2::Branch, BranchType)) -> Self {
//     let branch = b.0;
//     let upstream = match branch.upstream() {
//       Ok(u) => u.name().unwrap().unwrap().to_string(),
//       Err(_) => "".to_string(),
//     };
//     Self {
//       name: branch.name().unwrap().unwrap().to_string(),
//       branch_type: match b.1 {
//         BranchType::Local => "Local",
//         BranchType::Remote => "Remote",
//       }
//       .to_string(),
//       is_head: branch.is_head(),
//       upstream,
//     }
//   }
// }

// #[derive(Deserialize)]
// pub struct RepoRequest {
//   #[allow(dead_code)]
//   path: String,
//   #[allow(dead_code)]
//   absolute: bool,
// }

// pub async fn get_repo(req: web::Json<RepoRequest>) -> Json<Repo> {
//   let path = req
//     .path
//     .as_str()
//     .trim_end_matches(" ")
//     .trim_end_matches("/");
//   let name = path[path.rfind("/").unwrap_or(0) + 1..path.len()].to_string();

//   Json(Repo::new(
//     name,
//     match req.absolute {
//       true => path.to_string(),
//       false => format!(
//         "{}/{}",
//         get_home_dir(),
//         path
//           .trim_start_matches(" ")
//           .trim_start_matches("~")
//           .trim_start_matches("/")
//       ),
//     },
//   ))
// }

// pub async fn scan_local_repos() -> Json<Vec<Repo>> {
//   let home_dir = get_home_dir();
//   let root = Path::new(home_dir.as_str());
//   let repos = Vec::from_iter(
//     WalkDir::new(root)
//       .follow_links(false)
//       .into_iter()
//       .filter(|f| {
//         f.as_ref().is_ok_and(|d| {
//           d.file_type().is_dir() && d.file_name().eq_ignore_ascii_case(".git")
//         })
//       })
//       .map(|f| f.unwrap())
//       .map(|dir_entry| {
//         let dir = dir_entry
//           .path()
//           .parent()
//           .unwrap_or(root)
//           .to_string_lossy()
//           .to_string();
//         // Trimming the prefix from the path
//         let name = dir[dir.rfind("/").unwrap_or(0) + 1..dir.len()].to_string();
//         Repo::new(name, dir)
//       }),
//   );
//   web::Json(repos)
// }
