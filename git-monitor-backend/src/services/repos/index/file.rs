use git2::Repository;
use std::path::Path;

pub fn add_to_index<'a>(
  repo: &'a Repository,
  file_path: &Path,
) -> Result<(), String> {
  let ignored = repo.is_path_ignored(file_path);
  if ignored.is_ok() {
    let mut index = repo.index().unwrap();
    match index.add_path(file_path) {
      Ok(_) => match index.write() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Error Writing Index")),
      },
      Err(_) => Err(format!(
        "Error adding file '{}' to index",
        file_path.to_string_lossy().to_string()
      )),
    }
  } else {
    Err(format!(
      "File '{}' is ignored",
      file_path.to_string_lossy().to_string()
    ))
  }
}

pub fn remove_from_index<'a>(
  repo: &'a Repository,
  file_path: &Path,
) -> Result<(), String> {
  let ignored = repo.is_path_ignored(file_path);
  if ignored.is_ok() {
    let mut index = repo.index().unwrap();
    match index.remove_path(file_path) {
      Ok(_) => match index.write() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Error writing Index")),
      },
      Err(_) => Err(format!(
        "Error removing file '{}' from index",
        file_path.to_string_lossy().to_string()
      )),
    }
  } else {
    Err(format!(
      "File '{}' is ignored",
      file_path.to_string_lossy().to_string()
    ))
  }
}
