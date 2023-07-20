pub mod responses;

use std::path::Path;
use users::{get_current_uid, get_user_by_uid};
use walkdir::{DirEntry, WalkDir};

pub fn get_repos(root: &Path) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter(|f| {
            f.as_ref().is_ok_and(|d| {
                d.file_type().is_dir()
                    && d.file_name().eq_ignore_ascii_case(".git")
            })
        })
        .map(|f| f.unwrap())
}

pub fn get_username() -> String {
    get_user_by_uid(get_current_uid())
        .unwrap()
        .name()
        .to_string_lossy()
        .to_string()
}
