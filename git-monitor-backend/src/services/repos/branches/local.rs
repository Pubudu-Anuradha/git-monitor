use super::errors::BranchErrors;
use git2::{build::CheckoutBuilder, Branch, Repository};

pub fn create<'a>(
  repo: &'a Repository,
  branch_name: &str,
  checkout: &bool,
) -> Result<Branch<'a>, BranchErrors> {
  if !Branch::name_is_valid(branch_name).is_ok_and(|f| f) {
    Err(BranchErrors::MalformedName)
  } else if repo
    .find_branch(branch_name, git2::BranchType::Local)
    .is_ok()
    || repo
      .find_branch(branch_name, git2::BranchType::Remote)
      .is_ok()
  {
    Err(BranchErrors::BranchAlreadyExists)
  } else {
    let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
    match repo.branch(branch_name, &head_commit, false) {
      Ok(branch) => {
        if *checkout {
          let mut opts = CheckoutBuilder::new();
          let _ = repo.set_head(branch.get().name().unwrap());
          let __ = repo.checkout_head(Some(&mut opts));
        }
        Ok(branch)
      }
      Err(err) => {
        println!("{}", err.message());
        Err(BranchErrors::CreateBranchError)
      }
    }
  }
}
