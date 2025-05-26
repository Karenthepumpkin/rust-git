// Git 仓库核心模块
use std::fs::{create_dir, File};
use std::path::Path;

pub struct Repository {
    path: String, // 仓库路径
}
impl Repository {
    // 初始化新仓库
    pub fn init(path: &str) -> Self {
        // 创建 .git 目录结构
        println!("Initializing Git repository at {}", path);
        create_dir(".git").ok();
        create_dir(".git/objects").ok(); // 对象存储
        create_dir(".git/refs").ok();
        File::create(".git/HEAD").ok();
        Repository { path: path.to_string() }
    }
    // 验证Git仓库
    pub fn is_git_repo(_path: &str) -> bool {
        Path::new(".git").exists() // 检测.git目录
    }
}
