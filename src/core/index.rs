use crate::debug_log;
use crate::utils::hash::hashstr2path;
use core::hash;
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
    debug_log!("Start Merge conflict\n");
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
    pub fn load_merge(
        &mut self,
        path: Option<String>,
        path_merge: Option<String>,
        path_base: Option<String>,
    ) -> bool {
        self.staged_files.clear();
        let mut base_staged_files: HashMap<String, String> = HashMap::new();
        debug_log!(
            "load_merge : path : {} path_merge: {} path_base:{}",
            path.as_deref().unwrap_or("None"),
            path_merge.as_deref().unwrap_or("None"),
            path_base.as_deref().unwrap_or("None")
        );
        // 封装读取 index 文件的逻辑
        fn read_index(path: &str) -> Option<Vec<(String, String)>> {
            let file = match File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    debug_log!("Failed to open index file: {}", e);
                    // File::create(path).ok()?;
                    return None;
                }
            };
            let reader = BufReader::new(file);
            let mut entries = Vec::new();
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    let mut parts = line_content.rsplitn(2, '\t');
                    let hash = parts.next().unwrap_or("");
                    let path = parts.next().unwrap_or("");
                    if !path.is_empty() {
                        entries.push((path.to_string(), hash.to_string()));
                    } else {
                        debug_log!("Invalid line in index file: {}", line_content);
                    }
                }
            }
            Some(entries)
        }
        /* 
        if let Some(path_base) = path_base {
            let index_path = format!("{}/{}", self.repo_path, path_base);
            if let Some(entries) = read_index(&index_path) {
                for (path, hash) in entries {
                    base_staged_files.insert(path.clone(), hash.clone());
                    debug_log!("Loaded staged file from base: {} -> {}", path, hash);
                }
            } else {
                return false;
            }
        }
        */
        if let Some(path) = &path {
            let index_path = format!("{}/{}", self.repo_path, path);
            if let Some(entries) = read_index(&index_path) {
                for (path, hash) in entries {
                    self.staged_files.insert(path.clone(), hash.clone());
                    debug_log!("Loaded staged file from current: {} -> {}", path, hash);
                }
            } else {
                return false;
            }
        }

        // 3. 加载合并分支 path_merge
        if let Some(path_merge) = &path_merge {
            let index_path = format!("{}/{}", self.repo_path, path_merge);
            if let Some(entries) = read_index(&index_path) {
                for (path, hash) in entries {
                    if !self.staged_files.contains_key(&path) {
                        self.staged_files.insert(path.clone(), hash.clone());
                        debug_log!("Loaded staged file from merge: {} -> {}", path, hash);
                    } else if let Some(existing_hash) = self.staged_files.get(&path) {
                        if existing_hash != &hash {
                            // if let Some(base_hash) = base_staged_files.get(&path) {
                                // if base_hash != &hash {
                                    let path1 =
                                        crate::utils::hash::hashstr2path(existing_hash.to_string());
                                    let path1 = format!("{}/{}", self.repo_path, path1);
                                    let path2 =
                                        crate::utils::hash::hashstr2path(hash.to_string());                                   
                                    let path2 = format!("{}/{}", self.repo_path, path2);
                                    debug_log!("Merge conflict {} : {} -> {}",path, path1, path2);
                                    merge_conflict(path1, path2, path.clone());
                                    self.staged_files.insert(path.clone(), hash.clone());
                                    
                                // }
                            // }
                        }
                    }
                }
            } else {
                return false;
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
        for (path, hash) in &self.staged_files {
            let hash_path = format!("{}/{}", self.repo_path, hashstr2path(hash.clone()));
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
