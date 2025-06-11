use crate::core::repository::{self};
use crate::repo;
use crate::{debug_log};
pub fn init_command(path: &str) {
    // 初始化 Git 仓库
    // 创建 .git 目录结构
    // 创建对象存储目录
    // 创建引用存储目录
    // 创建 HEAD 文件
    if !repository::is_git_repo(path) {
        debug_log!("Git repository initialized.");
        repo!().init(path);
    } else {
        debug_log!("A Git repository already exists in this directory.");
        repo!().open(path);
    }
}
