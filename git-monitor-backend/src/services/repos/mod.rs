mod branches;
mod commit;
mod get;
mod index;
mod set;
mod updates;
mod utils;

use self::{
  branches::create_local_branch_from_head,
  index::{add_file_to_index, remove_file_from_index},
};
use actix_web::{
  web::{delete, get, post, resource, scope},
  Scope,
};
use get::{get_stored_repo_info, get_stored_repos_info};
use set::set_stored_repo_info;
use updates::update_repos_from_device;

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
    .service(
      resource("/repo/index/file")
        .route(post().to(add_file_to_index))
        .route(delete().to(remove_file_from_index)),
    )
}
