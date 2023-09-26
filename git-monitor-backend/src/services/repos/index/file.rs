use git2::Repository;
use std::path::Path;

pub fn add_to_index<'a>(
  repo: &'a Repository,
  file_path: &Path,
) -> Result<bool, String> {
  let ignored = repo.is_path_ignored(file_path);
  if ignored.is_ok() {
    let mut index = repo.index().unwrap();
    match index.add_path(file_path) {
      Ok(_) => match index.write() {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Error Writing Index: {}", e.message())),
      },
      Err(e) => Err(format!(
        "Error adding file '{}' to index: {}",
        file_path.to_string_lossy().to_string(),
        e.message()
      )),
    }
  } else {
    Err(format!(
      "File '{}' is ignored",
      file_path.to_string_lossy().to_string()
    ))
  }
}
