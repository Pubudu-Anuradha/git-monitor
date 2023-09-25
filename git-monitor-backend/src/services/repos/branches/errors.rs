use serde::Serialize;

#[derive(Serialize)]
pub enum BranchErrors {
  RepositoryNotFound,
  BranchAlreadyExists,
  MalformedName,
  CreateBranchError,
}
