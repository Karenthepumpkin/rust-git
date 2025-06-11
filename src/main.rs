use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::core::repository::{is_git_repo, Repository};

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
pub static REPO : Lazy<Mutex<Repository>> = Lazy::new(|| {
    Mutex::new(Repository::new("."))
});

fn main() {
    if is_git_repo(".") {
        repo!().open(".");
    }
    cli::command::git_execute();
}
