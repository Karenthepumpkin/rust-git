use crate::debug_log;
// Git 仓库核心模块
use super::blob::{self, BlobProcessor};
use super::commit::CommitBuilder;
use super::index::Index;
use super::tree::TreeBuilder;
use super::reference::Reference;
use std::fs::{File, create_dir};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
pub struct Repository {
    path: Arc<String>,             // 仓库路径
    blob_processor: BlobProcessor, // 数据对象处理器
    index: Index,
    tree: TreeBuilder,     // 树构建器
    commit: CommitBuilder, // 提交构建器
    reference: Reference,
}
impl Repository {
    // 初始化新仓库
    pub fn new(path: impl Into<String>) -> Self {
        let path = Arc::new(path.into());
        Repository {
            path: path.clone(),
            blob_processor: BlobProcessor::new(&path),
            index: Index::new(&path),
            tree: TreeBuilder::new(&path),
            commit: CommitBuilder::new(&path),
            reference: Reference::new(&path),
        }
    }
    pub fn init(&mut self, path: &str) {
        // 拼接路径并创建 .git 目录结构
        let git_dir = format!("{}/.git", path);
        let objects_dir = format!("{}/objects", git_dir);
        let refs_dir = format!("{}/refs", git_dir);
        let head_file = format!("{}/HEAD", git_dir);
        let index_file = format!("{}/index", git_dir);
        let refs_heads_dir = format!("{}/heads", refs_dir);
        create_dir(&git_dir).ok();
        create_dir(&objects_dir).ok(); // 对象存储
        create_dir(&refs_dir).ok();
        create_dir(&refs_heads_dir).ok();
        let master_ref = format!("{}/master", refs_heads_dir);
        File::create(&master_ref).ok();
        let mut head = File::create(&head_file).unwrap();
        writeln!(head, "ref: refs/heads/master").ok();
        for i in 0..=0xFF {
            let subdir = format!("{}/{}", objects_dir, format!("{:02x}", i));
            create_dir(&subdir).ok(); // 创建对象存储子目录
        }
        File::create(&index_file).ok();
        self.open(path);
    }
    // 打开现有仓库
    pub fn open(&mut self, path: &str) {
        if is_git_repo(path) {
            self.path = Arc::new(path.to_string());
            self.blob_processor.setpath(&self.path);
            self.index.load(&self.path);
        } else {
            debug_log!("Uninit");
            // panic!("Not a valid git repository: {}", path);
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
    pub fn commit(&mut self, message: &str) -> String {
        // 创建树对象
        let tree_hash = self.tree.create_tree(&self.index.get_tree());
        // 创建提交对象
        let current_branch = self.reference.get_current_branch();
        let parent_commit = if let Some(ref branch) = current_branch {
            self.reference.get_last_commit(branch)
        } else {
            None
        };
        let commit_hash = self
            .commit
            .create_commit(&tree_hash, parent_commit.as_deref(), message);
        if let Some(current_branch) = current_branch {
            self.reference.set_last_commit(&current_branch, &commit_hash);
        }
        commit_hash
    }
    pub fn exit(&self) {
        // 保存索引
        if !self.index.save() {
            debug_log!("Failed to save index");
        }
    }
}
pub fn is_git_repo(path: &str) -> bool {
    let git_dir = format!("{}/.git", path);
    Path::new(&git_dir).exists() // 检测.git目录
}
