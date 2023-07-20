use serde::Serialize;

#[derive(Serialize)]
pub struct Repo {
    pub name: String,
    pub dir: String,
}

#[derive(Serialize)]
pub struct Repos {
    pub repos: Vec<Repo>,
}
