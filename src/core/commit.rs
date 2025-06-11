use crate::debug_log;
use std::{fmt::format, fs, sync::Arc};
pub struct CommitBuilder {
    repo_path: Arc<String>,
}
impl CommitBuilder {
    /// 创建新提交对象
    pub fn new(repo_path: &Arc<String>) -> Self {
        CommitBuilder {
            repo_path: Arc::clone(repo_path),
        }
    }
    pub fn create_commit(&self, tree_hash: &str, parent_commit: Option<&str>, commit_message: &str) -> String {
        crate::core::object::save(
            crate::core::object::Object::Commit(
                format!(
                    "tree {}\n{}message: {}",
                    tree_hash,
                    parent_commit
                        .map_or("parent None\n".to_string(), |p| format!("parent {}\n", p)),
                    commit_message
                )
            ),
            self.repo_path.as_str(),
        )
    }
}

