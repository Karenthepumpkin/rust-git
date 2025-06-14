use crate::commands::branch;
use crate::debug_log;
use crate::utils::hash::{hash2path, hashstr2path};
// Git 仓库核心模块
use super::blob::{self, BlobProcessor};
use super::commit::CommitBuilder;
use super::index::Index;
use super::reference::Reference;
use super::tree::TreeBuilder;
use std::fs::{File, create_dir};
use std::io::{BufRead, BufReader, Write};
use std::path::{self, Path};
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
            // self.blob_processor.delete_blob(&hash);
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
            self.reference
                .set_last_commit(&current_branch, &commit_hash);
        }
        commit_hash
    }
    pub fn merge_commit(
        &mut self,
        parent_commit: Option<String>,
        path_merge: Option<String>,
    ) -> String {
        // 创建树对象
        let tree_hash = self.tree.create_tree(&self.index.get_tree());
        // 创建提交对象
        let current_branch = self.reference.get_current_branch();
        let commit_hash =
            self.commit
                .merge_commit(&tree_hash, parent_commit.as_deref(), path_merge.as_deref());
        if let Some(current_branch) = current_branch {
            self.reference
                .set_last_commit(&current_branch, &commit_hash);
        }
        debug_log!("merge_commit {}", commit_hash);
        commit_hash
    }
    pub fn new_branch(&mut self, branch_name: &str) -> bool {
        let current_branch = self.reference.get_current_branch();
        let last_commit = if let Some(ref branch) = current_branch {
            self.reference.get_last_commit(branch)
        } else {
            None
        };
        if last_commit.is_none() {
            debug_log!("No commits found to create a new branch");
            return false;
        }
        self.reference
            .new_branch(branch_name, &last_commit.unwrap());
        true
    }
    pub fn delete_branch(&mut self, branch_name: &str) {
        self.reference.delete_branch(branch_name);
    }
    pub fn set_current_branch(&mut self, branch_name: &str) -> bool {
        if self.reference.set_current_branch(branch_name) {
            debug_log!("Switched to branch: {}", branch_name);
        } else {
            debug_log!("Failed to switch to branch: {}", branch_name);
            return false;
        }
        let commit_path_opt = self.reference.get_current_branch();
        let commit_path = match commit_path_opt {
            Some(ref path) => path,
            None => {
                debug_log!("No current branch found");
                return false;
            }
        };
        let file = match File::open(commit_path) {
            Ok(f) => f,
            Err(_) => {
                debug_log!("Failed to open commit path file {}", commit_path);
                return false;
            }
        };
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        if let Ok(n) = reader.read_line(&mut first_line) {
            if n > 0 {
                if let Some(_tree_hash) = first_line.strip_prefix("tree ") {
                    self.index.load_from(_tree_hash.to_string());
                }
            }
        }
        true
    }
    pub fn exit(&self) {
        // 保存索引
        if !self.index.save() {
            debug_log!("Failed to save index");
        }
    }
    pub fn refresh(&self) {
        self.index.refresh();
    }
    pub fn get_all_ancestors(&self, mut commit: String) -> std::collections::HashSet<String> {
        let mut ancestors = std::collections::HashSet::new();
        while !commit.is_empty() && ancestors.insert(commit.clone()) {
            if let Some(parent) = self.reference.get_father_commit(&commit) {
                commit = parent;
            } else {
                break;
            }
        }
        ancestors
    }
    pub fn merge(&mut self, merge_branch: String) {
        // 1. 获取当前分支和目标分支的最后一次提交
        debug_log!("START MERGE\n\n\n");

        let current_branch = match self.reference.get_current_branch() {
            Some(b) => b,
            None => {
                debug_log!("No current branch");
                return;
            }
        };

        let current_commit = match self.reference.get_last_commit(&current_branch) {
            Some(c) => c,
            None => {
                debug_log!("No commit on current branch");
                return;
            }
        };
        let merge_commit = match self.reference.get_last_commit(&merge_branch) {
            Some(c) => c,
            None => {
                debug_log!("No commit on merge branch");
                return;
            }
        };
        let current_ancestors = self.get_all_ancestors(current_commit.clone());
        let mut base_commit = None;
        let mut merge_iter = merge_commit.clone();
        while !merge_iter.is_empty() {
            if current_ancestors.contains(&merge_iter) {
                base_commit = Some(merge_iter.clone());
                break;
            }
            if let Some(parent) = self.reference.get_father_commit(&merge_iter) {
                merge_iter = parent;
            } else {
                break;
            }
        }
        let base_branch = match &base_commit {
            Some(b) => b,
            None => {
                debug_log!("No common ancestor found for merge");
                return;
            }
        };
        debug_log!("merge base: {}", base_branch);
        // 2. 获取 tree hash
        let get_tree_hash = |commit_hash: &str| -> Option<String> {
            use std::fs::File;
            use std::io::{BufRead, BufReader};
            let (dir, file) = commit_hash.split_at(2);
            let commit_path = format!("{}/.git/objects/{}/{}", self.path, dir, file);
            let file = File::open(&commit_path).ok()?;
            let mut reader = BufReader::new(file);
            let mut first_line = String::new();
            reader.read_line(&mut first_line).ok()?;
            first_line
                .strip_prefix("tree ")
                .map(|s| s.trim().to_string())
        };
        let current_tree = get_tree_hash(&current_commit);
        let merge_tree = get_tree_hash(&merge_commit);
        let base_tree = match &base_commit {
            Some(hash) => get_tree_hash(hash),
            None => None,
        };
        let current_tree_path = match current_tree {
            Some(path) => Some(format!("{}/{}", self.path, hashstr2path(path))),
            None => None,
        };
        let merge_tree_path = match merge_tree {
            Some(path) => Some(format!("{}/{}", self.path, hashstr2path(path))),
            None => None,
        };
        let base_tree_path = match base_tree {
            Some(path) => Some(format!("{}/{}", self.path, hashstr2path(path))),
            None => None,
        };
        self.index
            .load_merge(current_tree_path, merge_tree_path, base_tree_path);
        self.index.refresh();
        self.merge_commit(
            self.reference.get_last_commit(&current_branch),
            self.reference.get_last_commit(&merge_branch),
        );
    }
}
pub fn is_git_repo(path: &str) -> bool {
    let git_dir = format!("{}/.git", path);
    Path::new(&git_dir).exists() // 检测.git目录
}
