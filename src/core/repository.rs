// Git 仓库核心模块
use super::blob::{self, BlobProcessor};
use super::index::Index;
use std::fs::{File, create_dir};
use std::path::Path;
use std::sync::Arc;
pub struct Repository {
    path: Arc<String>,             // 仓库路径
    blob_processor: BlobProcessor, // 数据对象处理器
    index: Index,
}
impl Repository {
    // 初始化新仓库
    pub fn new(path: impl Into<String>) -> Self {
        let path = Arc::new(path.into());
        Repository {
            path: path.clone(),
            blob_processor: BlobProcessor::new(&path),
            index: Index::new(&path),
        }
    }
    pub fn init(&mut self,path: &str) {
        // 拼接路径并创建 .git 目录结构
        let git_dir = format!("{}/.git", path);
        let objects_dir = format!("{}/objects", git_dir);
        let refs_dir = format!("{}/refs", git_dir);
        let head_file = format!("{}/HEAD", git_dir);
        let index_file = format!("{}/index", git_dir);
        create_dir(&git_dir).ok();
        create_dir(&objects_dir).ok(); // 对象存储
        for i in 0..=0xFF {
            let subdir = format!("{}/{}", objects_dir, format!("{:02x}", i));
            create_dir(&subdir).ok(); // 创建对象存储子目录
        }
        create_dir(&refs_dir).ok();
        File::create(&head_file).ok();
        File::create(&index_file).ok();
        self.open(path);
    }
    // 验证Git仓库

    // 打开现有仓库
    pub fn open(&mut self ,path: &str) {
        if is_git_repo(path) {
            self.path = Arc::new(path.to_string());
            self.blob_processor.setpath(&self.path);
            self.index.load(&self.path);
        } else {
            panic!("Not a valid git repository: {}", path);
        }
    }
}
impl Repository {
    pub fn stage_file(&mut self, path: &str) {
        if let Some(hash) = self.index.unstage_file(path) {
            self.blob_processor.delete_blob(&hash);
        }
        let hash = self.blob_processor.create_blob(path);
        self.index.stage_file(path, &hash);
    }
    pub fn unstage_file(&mut self, path: &str) {
        let hash = self.index.unstage_file(path);
        if let Some(hash) = hash {
            self.blob_processor.delete_blob(&hash);
        }
    }
}
pub fn is_git_repo(path: &str) -> bool {
    let git_dir = format!("{}/.git", path);
    Path::new(&git_dir).exists() // 检测.git目录
}
