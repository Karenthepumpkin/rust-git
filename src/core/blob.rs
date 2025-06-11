use crate::debug_log;
use std::{fs, sync::Arc};
pub struct BlobProcessor {
    repo_path: Arc<String>,
}

impl BlobProcessor {
    /// 创建Blob对象
    pub fn new(repo_path: &Arc<String>) -> Self {
        BlobProcessor {
            repo_path: Arc::clone(repo_path),
        }
    }
    pub fn create_blob(&self, path: &str) -> String {
        //TODO: Implement actual blob creation logic
        debug_log!("Creating blob with path: {}", path);
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                debug_log!("Failed to read file {}: {}", path, e);
                return String::new();
            }
        };
        crate::core::object::save(
            crate::core::object::Object::Blob(content.to_string()),
            self.repo_path.as_str(),
        )
    }
    pub fn delete_blob(&self, hash: &str) -> bool {
        let path = format!("{}/objects/{}", self.repo_path, hash);
        match fs::remove_file(&path) {
            Ok(_) => {
                debug_log!("Deleted blob: {}", hash);
                true
            }
            Err(e) => {
                debug_log!("Failed to delete blob {}: {}", hash, e);
                false
            }
        }
    }
    pub fn setpath(&mut self, path: &Arc<String>) {
        self.repo_path = Arc::clone(path);
    }
}
