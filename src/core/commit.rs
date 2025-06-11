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
    pub fn create_commit(
        &self,
        tree_hash: &str,
        parent_commit: Option<&str>,
        commit_message: &str,
    ) -> String {
        crate::core::object::save(
            crate::core::object::Object::Commit(format!(
                "tree {}\n{}message: {}",
                tree_hash,
                parent_commit.map_or("parent None\n".to_string(), |p| format!("parent {}\n", p)),
                commit_message
            )),
            self.repo_path.as_str(),
        )
    }
    pub fn merge_commit(
        &self,
        tree_hash: &str,
        parent_commit: Option<&str>,
        parent_commit_merge: Option<&str>,
    ) -> String {
        let commit_message = if let Some(branch_name) = parent_commit_merge {
            format!("Merge branch '{}'", branch_name)
        } else {
            String::from("Merge commit")
        };
        crate::core::object::save(
            crate::core::object::Object::Commit(format!(
                "tree {}\n{}message: {}",
                tree_hash,
                match (parent_commit, parent_commit_merge) {
                    (Some(p1), Some(p2)) => format!("parent {} {}\n", p1, p2),
                    (Some(p1), None) => format!("parent {}\n", p1),
                    (None, Some(p2)) => format!("parent {}\n", p2),
                    (None, None) => "parent None\n".to_string(),
                },
                commit_message
            )),
            self.repo_path.as_str(),
        )
    }
}
