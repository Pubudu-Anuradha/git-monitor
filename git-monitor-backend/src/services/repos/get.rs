use crate::{
  get_prisma_connection,
  prisma::{self, repo},
  services::user::get_home_dir,
};
use actix_web::web::Json;
use serde::Deserialize;

pub async fn get_stored_repos_info(
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

#[derive(Deserialize)]
pub struct RepoRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
}

pub async fn get_stored_repo_info(
  req: Json<RepoRequest>,
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
