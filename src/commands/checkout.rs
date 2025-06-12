use super::branch;
use crate::{commands::branch::branch_command, repo};
pub fn checkout_command(name: &str, new_branch: bool) {
    if new_branch {
        branch_command(name, branch::BranchCommandType::New);
    }
    repo!().set_current_branch(name);
}
