use crate::debug_log;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::{File, read, write};
use std::io::{BufRead, BufReader, Write};
use std::option;
use std::{fs, sync::Arc};
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
        self.staged_files.clear();
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
    pub fn load_from(&mut self, path: String) -> bool {
        self.staged_files.clear();
        let index_path = format!("{}/{}", self.repo_path, path);
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
    pub fn load_merge(&mut self, path: String, path_merge: String, path_base: String) -> bool {
        self.staged_files.clear();
        let mut base_staged_files: HashMap<String, String> = HashMap::new();
        let index_path = format!("{}/{}", self.repo_path, path_base);
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
                    base_staged_files.insert(path.to_string(), hash.to_string());
                    debug_log!("Loaded staged file: {} -> {}", path, hash);
                } else {
                    debug_log!("Invalid line in index file: {}", line_content);
                }
            }
        }

        let index_path = format!("{}/{}", self.repo_path, path);
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
        let index_path = format!("{}/{}", self.repo_path, path_merge);
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
                    if !self.staged_files.contains_key(path) || base_staged_files.contains_key(path)
                    {
                        self.staged_files.insert(path.to_string(), hash.to_string());
                        debug_log!("Loaded staged file: {} -> {}", path, hash);
                    } else {
                        debug_log!("Invalid line in index file: {}", line_content);
                    }
                }
            }
        }
        true
    }
    pub fn save(&self) -> bool {
        let index_path = format!("{}/.git/index", self.repo_path);
        let mut file = match File::create(&index_path) {
            Ok(f) => f,
            Err(e) => {
                debug_log!("Failed to create index file: {}", e);
                return false;
            }
        };
        for (path, hash) in &self.staged_files {
            if let Err(e) = writeln!(file, "{}\t{}", path, hash) {
                debug_log!("Failed to write to index file: {}", e);
                return false;
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
    pub fn get_tree(&self) -> String {
        self.staged_files
            .iter()
            .map(|(path, hash)| format!("{}\t{}", path, hash))
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub fn refresh(&self) {
        for (path, hash_path) in &self.staged_files {
            match fs::read(&hash_path) {
                Ok(content) => {
                    if let Err(e) = write(path, &content) {
                        debug_log!("Failed to write file {}: {}", path, e);
                    } else {
                        debug_log!("Restored file {} from blob {}", path, hash_path);
                    }
                }
                Err(e) => {
                    debug_log!("Blob not found for hash {}: {}", hash_path, e);
                }
            }
        }
    }
}
