use crate::debug_log;
use std::{fs, sync::Arc};
pub struct Reference {
    repo_path: Arc<String>,
}

impl Reference {
    /// 创建Blob对象
    pub fn new(repo_path: &Arc<String>) -> Self {
        Reference {
            repo_path: Arc::clone(repo_path),
        }
    }
    pub fn get_last_commit(&self, ref_name: &str) -> Option<String> {
        let ref_path = format!("{}/refs/heads/{}", self.repo_path, ref_name);
        match fs::read_to_string(&ref_path) {
            Ok(content) => {
                debug_log!("Read last commit from {}: {}", ref_path, content);
                Some(content.trim().to_string())
            }
            Err(e) => {
                debug_log!("Failed to read last commit from {}: {}", ref_path, e);
                None
            }
        }
    }
    pub fn get_current_branch(&self) -> Option<String> {
        let head_path = format!("{}/.git/HEAD", self.repo_path);
        match fs::read_to_string(&head_path) {
            Ok(content) => {
                if content.starts_with("ref: ") {
                    let ref_name = content.trim().strip_prefix("ref: ").unwrap_or("");
                    let branch_name = ref_name.split('/').last().unwrap_or("");
                    debug_log!("Current branch is: {}", branch_name);
                    Some(branch_name.to_string())
                } else {
                    debug_log!("HEAD is not a reference, content: {}", content);
                    None
                }
            }
            Err(e) => {
                debug_log!("Failed to read HEAD file: {}", e);
                None
            }
        }
    }
    pub fn set_last_commit(&self, ref_name: &str, commit_hash: &str) -> bool {
        let ref_path = format!("{}/.git/refs/heads/{}", self.repo_path, ref_name);
        match fs::write(&ref_path, commit_hash) {
            Ok(_) => {
                debug_log!("Set last commit for {} to {}", ref_path, commit_hash);
                true
            }
            Err(e) => {
                debug_log!("Failed to set last commit for {}: {}", ref_path, e);
                false
            }
        }
    }
    pub fn setpath(&mut self, path: &Arc<String>) {
        self.repo_path = Arc::clone(path);
    }
}
