use crate::debug_log;
use std::path::Path;
use std::{fs, sync::Arc};
pub struct Reference {
    repo_path: Arc<String>,
}

impl Reference {
    pub fn new(repo_path: &Arc<String>) -> Self {
        Reference {
            repo_path: Arc::clone(repo_path),
        }
    }
    pub fn get_last_commit(&self, ref_name: &str) -> Option<String> {
        let ref_path = format!("{}/.git/refs/heads/{}", self.repo_path, ref_name);
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
    pub fn new_branch(&self, branch_name: &str, commit_hash: &str) -> bool {
        let ref_path = format!("{}/.git/refs/heads/{}", self.repo_path, branch_name);
        match fs::write(&ref_path, commit_hash) {
            Ok(_) => {
                debug_log!(
                    "Created new branch {} with commit {}",
                    branch_name,
                    commit_hash
                );
                true
            }
            Err(e) => {
                debug_log!("Failed to create new branch {}: {}", branch_name, e);
                false
            }
        }
    }
    pub fn delete_branch(&self, branch_name: &str) -> bool {
        let ref_path = format!("{}/.git/refs/heads/{}", self.repo_path, branch_name);
        let ref_path = Path::new(&ref_path);

        if ref_path.exists() {
            match fs::remove_file(ref_path) {
                Ok(_) => {
                    debug_log!("remove {}", branch_name);
                    true
                }
                Err(e) => false,
            }
        } else {
            debug_log!("{} not exist", branch_name);
            false
        }
    }
    pub fn set_current_branch(&self, branch_name: &str) -> bool {
        let head_path = format!("{}/.git/HEAD", self.repo_path);
        match fs::write(&head_path, format!("ref: refs/heads/{}", branch_name)) {
            Ok(_) => {
                debug_log!("Set HEAD to branch {}", branch_name);
                true
            }
            Err(e) => {
                debug_log!("Failed to set HEAD to branch {}: {}", branch_name, e);
                false
            }
        }
    }
    pub fn get_father_commit(&self, commit_hash: &str) -> Option<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let (dir, file) = commit_hash.split_at(2);
        let commit_path = format!("{}/.git/objects/{}/{}", self.repo_path, dir, file);

        let file = match File::open(&commit_path) {
            Ok(f) => f,
            Err(e) => {
                debug_log!("Failed to open commit object {}: {}", commit_path, e);
                return None;
            }
        };
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some(parent) = line.strip_prefix("parent ") {
                return Some(parent.trim().to_string());
            }
        }
        None
    }
    pub fn setpath(&mut self, path: &Arc<String>) {
        self.repo_path = Arc::clone(path);
    }
}
