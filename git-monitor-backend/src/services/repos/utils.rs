use git2::{RepositoryState, StatusEntry};

pub fn state_to_string(state: RepositoryState) -> String {
  match state {
    RepositoryState::Clean => "Clean",
    RepositoryState::Merge => "Merge",
    RepositoryState::Revert => "Revert",
    RepositoryState::RevertSequence => "RevertSequence",
    RepositoryState::CherryPick => "CherryPick",
    RepositoryState::CherryPickSequence => "CheeryPickSequence",
    RepositoryState::Bisect => "Bisect",
    RepositoryState::Rebase => "Rebase",
    RepositoryState::RebaseInteractive => "RebaseInteractive",
    RepositoryState::RebaseMerge => "RebaseMerge",
    RepositoryState::ApplyMailbox => "ApplyMailbox",
    RepositoryState::ApplyMailboxOrRebase => "ApplyMailboxOrRebase",
  }
  .to_string()
}

#[allow(unused)]
pub async fn status(entry: StatusEntry<'_>) -> (String, String) {
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
  (status, path)
}
