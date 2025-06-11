use crate::core::repository::{Repository, is_git_repo};
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod cli;
mod commands;
mod core;
mod utils;

#[macro_export]
macro_rules! repo {
    () => {
        $crate::REPO.lock().unwrap()
    };
}

// 创建全局可变 Repository
pub static REPO: Lazy<Mutex<Repository>> = Lazy::new(|| Mutex::new(Repository::new(".")));

fn main() {
    if is_git_repo(".") {
        repo!().open(".");
    }
    cli::command::git_execute();
    repo!().exit();
}
