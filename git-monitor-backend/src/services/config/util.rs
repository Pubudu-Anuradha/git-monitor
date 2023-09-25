pub fn get_credentials() -> (String, String) {
  let conf = git2::Config::open_default().unwrap();
  let email = conf.get_string("user.email").unwrap();
  let uname = conf.get_string("user.name").unwrap();
  (email, uname)
}
