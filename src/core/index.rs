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
pub fn merge_conflict(path1: String, path2: String, filename: String) {
    use std::fs;
    use std::io::{BufRead, BufReader};

    let file1 = match fs::File::open(&path1) {
        Ok(f) => f,
        Err(e) => {
            debug_log!("Failed to open {}: {}", path1, e);
            return;
        }
    };
    let file2 = match fs::File::open(&path2) {
        Ok(f) => f,
        Err(e) => {
            debug_log!("Failed to open {}: {}", path2, e);
            return;
        }
    };

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let lines1: Vec<String> = reader1.lines().filter_map(Result::ok).collect();
    let lines2: Vec<String> = reader2.lines().filter_map(Result::ok).collect();

    let max_len = lines1.len().max(lines2.len());

    let mut conflict_start: Option<usize> = None;

    for i in 0..max_len {
        let l1 = lines1.get(i);
        let l2 = lines2.get(i);
        let conflict = match (l1, l2) {
            (Some(a), Some(b)) if a != b => true,
            (Some(_), None) | (None, Some(_)) => true,
            _ => false,
        };

        if conflict {
            if conflict_start.is_none() {
                conflict_start = Some(i + 1);
            }
        } else if let Some(start) = conflict_start {
            if start == i {
                println!("Merge conflict in {}:{}", filename, start);
            } else {
                println!("Merge conflict in {}:[{}, {}]", filename, start, i);
            }
            conflict_start = None;
        }
    }
    // 处理结尾冲突
    if let Some(start) = conflict_start {
        if start == max_len {
            println!("Merge conflict in {}:{}", filename, start);
        } else {
            println!("Merge conflict in {}:[{}, {}]", filename, start, max_len);
        }
    }
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
