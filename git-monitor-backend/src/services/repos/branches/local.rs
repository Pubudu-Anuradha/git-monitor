use git2::{build::CheckoutBuilder, Branch, Repository};

/// checkout: checkout to branch after creation?
pub fn create<'a>(
  repo: &'a Repository,
  branch_name: &str,
  checkout: &bool,
) -> Result<(Branch<'a>, Result<bool, String>), String> {
  if !Branch::name_is_valid(branch_name).is_ok_and(|f| f) {
    Err(format!("Malformed branch name '{}'", branch_name))
  } else if repo
    .find_branch(branch_name, git2::BranchType::Local)
    .is_ok()
    || repo
      .find_branch(branch_name, git2::BranchType::Remote)
      .is_ok()
  {
    Err(format!("Branch with name '{}' already exists", branch_name))
  } else {
    match repo.head() {
      Ok(head) => match head.peel_to_commit() {
        Ok(head_commit) => {
          // Creating Branch
          match repo.branch(branch_name, &head_commit, false) {
            Ok(branch) => {
              if *checkout {
                match repo.set_head(branch.get().name().unwrap()) {
                  Ok(_) => {
                    let mut opts = CheckoutBuilder::new();
                    let checked_out = repo.checkout_head(Some(&mut opts));
                    match checked_out {
                      Ok(_) => Ok((branch, Ok(true))),
                      Err(e) => Ok((
                        branch,
                        Err(format!("Error checking out: {}", e.message())),
                      )),
                    }
                  }
                  Err(e) => Ok((
                    branch,
                    Err(format!("Error setting HEAD: {}", e.message())),
                  )),
                }
              } else {
                Ok((branch, Ok(false)))
              }
            }
            Err(e) => Err(format!(
              "Error creating branch '{}': {}",
              branch_name,
              e.message()
            )),
          }
        }
        Err(e) => Err(format!(
          "Couldn't find a commit pointed to by HEAD: {}",
          e.message()
        )),
      },
      Err(e) => Err(format!("Couldn't access HEAD: {}", e.message())),
    }
  }
}
