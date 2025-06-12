use crate::core::repository::{self, Repository};
use crate::repo;
use crate::{REPO, debug_log};
pub fn merge_command(merge_branch: &str) {
    repo!().merge(merge_branch.to_string());
}
