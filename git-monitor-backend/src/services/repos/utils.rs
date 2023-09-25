use crate::services::user::get_home_dir;

pub fn repository_state_to_string(state: git2::RepositoryState) -> String {
  match state {
    git2::RepositoryState::Clean => "Clean",
    git2::RepositoryState::Merge => "Merge",
    git2::RepositoryState::Revert => "Revert",
    git2::RepositoryState::RevertSequence => "RevertSequence",
    git2::RepositoryState::CherryPick => "CherryPick",
    git2::RepositoryState::CherryPickSequence => "CheeryPickSequence",
    git2::RepositoryState::Bisect => "Bisect",
    git2::RepositoryState::Rebase => "Rebase",
    git2::RepositoryState::RebaseInteractive => "RebaseInteractive",
    git2::RepositoryState::RebaseMerge => "RebaseMerge",
    git2::RepositoryState::ApplyMailbox => "ApplyMailbox",
    git2::RepositoryState::ApplyMailboxOrRebase => "ApplyMailboxOrRebase",
  }
  .to_string()
}

pub fn status_entry_to_string_pair(
  entry: git2::StatusEntry<'_>,
) -> (String, String) {
  let status = match entry.status() {
    git2::Status::CURRENT => "CURRENT",
    git2::Status::INDEX_NEW => "INDEX_NEW",
    git2::Status::INDEX_MODIFIED => "INDEX_MODIFIED",
    git2::Status::INDEX_DELETED => "INDEX_DELETED",
    git2::Status::INDEX_RENAMED => "INDEX_RENAMED",
    git2::Status::INDEX_TYPECHANGE => "INDEX_TYPECHANGE",
    git2::Status::WT_NEW => "WT_NEW",
    git2::Status::WT_MODIFIED => "WT_MODIFIED",
    git2::Status::WT_DELETED => "WT_DELETED",
    git2::Status::WT_TYPECHANGE => "WT_TYPECHANGE",
    git2::Status::WT_RENAMED => "WT_RENAMED",
    git2::Status::IGNORED => "IGNORED",
    git2::Status::CONFLICTED => "CONFLICTED",
    _ => "UNKNOWN",
  }
  .to_string();

  let path = match entry.path() {
    Some(p) => p,
    None => "UNKNOWN",
  }
  .to_string();
  (status.clone(), path.clone())
}

pub fn absolute_path(path: &String, absolute: bool) -> String {
  let _path = path.as_str().trim_end_matches(" ").trim_end_matches("/");
  let abs_path = match absolute {
    true => _path.to_string(),
    false => format!(
      "{}/{}",
      get_home_dir(),
      _path
        .trim_start_matches(" ")
        .trim_start_matches("~")
        .trim_start_matches("/")
    ),
  };
  abs_path
}
