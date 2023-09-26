use super::utils::absolute_path;
use actix_web::web::Json;
use git2::Repository;
use serde::Deserialize;
use std::path::Path;

mod file;

#[derive(Deserialize)]
pub struct IndexFileRequest {
  #[allow(dead_code)]
  path: String,
  #[allow(dead_code)]
  absolute: bool,
  #[allow(dead_code)]
  file_path: String,
}

pub async fn add_file_to_index(
  req: Json<IndexFileRequest>,
) -> Json<Result<bool, String>> {
  let abs_path = absolute_path(&req.path, req.absolute);
  let repo = Repository::open(Path::new(abs_path.as_str()));
  Json(match repo {
    Ok(repo) => {
      let file = Path::new(&req.file_path);
      file::add_to_index(&repo, file)
    }
    Err(e) => Err(format!(
      "Error accessing repository @\"{}\": {}",
      abs_path,
      e.message()
    )),
  })
}
