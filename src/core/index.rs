use crate::debug_log;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option;
use std::{os::unix::fs, sync::Arc};
pub struct Index {
    repo_path: Arc<String>,
    staged_files: HashMap<String, String>,
}
impl Index {
    pub fn new(repo_path: &Arc<String>) -> Self {
        Index {
            repo_path: Arc::clone(repo_path),
            staged_files: HashMap::new(),
        }
    }
    pub fn load(&mut self, path: &Arc<String>) -> bool {
        self.repo_path = path.clone();
        let index_path = format!("{}/.git/index", self.repo_path);
        let file = match File::open(&index_path) {
            Ok(f) => f,
            Err(e) => {
                debug_log!("Failed to open index file: {}", e);
                File::create(&index_path).ok();
                return false;
            }
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            // 每一行内容为 path /t hash
            if let Ok(line_content) = line {
                let mut parts = line_content.rsplitn(2, '\t');
                let hash = parts.next().unwrap_or("");
                let path = parts.next().unwrap_or("");
                if !path.is_empty() {
                    self.staged_files.insert(path.to_string(), hash.to_string());
                    debug_log!("Loaded staged file: {} -> {}", path, hash);
                } else {
                    debug_log!("Invalid line in index file: {}", line_content);
                }
            }
        }
        true
    }

    pub fn stage_file(&mut self, path: &str, hash: &str) {
        self.staged_files.insert(path.to_string(), hash.to_string());
    }
    pub fn unstage_file(&mut self, path: &str) -> Option<String> {
        self.staged_files.remove(path)
    }
}
