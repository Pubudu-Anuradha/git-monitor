mod errors;
mod local;

use self::errors::BranchErrors;
use super::utils::absolute_path;
use actix_web::web::Json;
use git2::Repository;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct CreateLocalBranchRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
  #[allow(dead_code)]
  new_branch_name: String,
  #[allow(dead_code)]
  checkout: bool,
}

pub async fn create_local_branch_from_head<'a>(
  req: Json<CreateLocalBranchRequest>,
) -> Json<Result<bool, BranchErrors>> {
  let abs_path = absolute_path(&req.path, req.absolute);
  let repo = Repository::open(Path::new(abs_path.as_str()));
  if repo.is_ok() {
    Json(
      match local::create(&repo.unwrap(), &req.new_branch_name, &req.checkout) {
        Ok(branch) => Ok(branch.name().is_ok()),
        Err(err) => Err(err),
      },
    )
  } else {
    Json(Err(BranchErrors::RepositoryNotFound))
  }
}
