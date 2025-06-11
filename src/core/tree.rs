use crate::debug_log;
use std::{fmt::format, fs, sync::Arc};
pub struct TreeBuilder {
    repo_path: Arc<String>,
}
impl TreeBuilder {
    /// 创建新提交对象
    pub fn new(repo_path: &Arc<String>) -> Self {
        TreeBuilder {
            repo_path: Arc::clone(repo_path),
        }
    }
    pub fn create_tree(&self, include_file: &str) -> String {
        crate::core::object::save(
            crate::core::object::Object::Tree(include_file.to_string()),
            self.repo_path.as_str(),
        )
    }
}
