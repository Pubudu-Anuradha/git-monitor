mod branches;
mod get;
mod set;
mod updates;
mod utils;

use actix_web::{
  web::{get, post, resource, scope},
  Scope,
};
use get::{get_stored_repo_info, get_stored_repos_info};
use set::set_stored_repo_info;
use updates::update_repos_from_device;

use self::branches::create_local_branch_from_head;

pub fn repo_scope() -> Scope {
  scope("repos")
    .service(resource("").to(get_stored_repos_info))
    .service(resource("/update").to(update_repos_from_device))
    .service(
      resource("/repo")
        .route(get().to(get_stored_repo_info))
        .route(post().to(set_stored_repo_info)),
    )
    .service(
      resource("/repo/branch").route(
        post() // Create New Branch
          .to(create_local_branch_from_head),
      ),
    )
}
