use super::updates::update_repo_from_device;
use crate::{
  get_prisma_connection, prisma::repo, services::user::get_home_dir,
};
use actix_web::web::Json;
use prisma_client_rust::QueryError;

#[derive(serde::Deserialize)]
pub struct SetRepoRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
  #[allow(dead_code)]
  managed: bool,
}

pub async fn set_stored_repo_info(
  req: Json<SetRepoRequest>,
) -> Json<Result<Option<repo::Data>, QueryError>> {
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
  let __ = update_repo_from_device(abs_path.clone()).await;
  let _ = conn
    .repo()
    .update(
      repo::UniqueWhereParam::DirEquals(abs_path.clone()),
      vec![repo::managed::set(req.managed)],
    )
    .exec()
    .await;
  Json(
    conn
      .repo()
      .find_unique(repo::UniqueWhereParam::DirEquals(abs_path))
      .with(repo::branches::fetch(vec![]))
      .with(repo::statuses::fetch(vec![]))
      .exec()
      .await,
  )
}
