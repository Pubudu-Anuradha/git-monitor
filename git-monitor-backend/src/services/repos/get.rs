use super::utils::absolute_path;
use crate::{
  get_prisma_connection,
  prisma::{self, repo},
};
use actix_web::{
  web::{Json, Query},
  HttpRequest,
};
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
  req: HttpRequest,
) -> Json<Result<Option<repo::Data>, prisma_client_rust::QueryError>> {
  let params = Query::<RepoRequest>::from_query(req.query_string()).unwrap();
  let conn = get_prisma_connection().await;
  let abs_path = absolute_path(&params.path, params.absolute);
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
