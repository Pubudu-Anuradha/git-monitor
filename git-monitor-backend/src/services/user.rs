use users::{get_current_uid, get_user_by_uid};

pub fn get_username() -> String {
  get_user_by_uid(get_current_uid())
    .unwrap()
    .name()
    .to_string_lossy()
    .to_string()
}

pub fn get_home_dir() -> String {
  format!("/home/{}", get_username())
}
