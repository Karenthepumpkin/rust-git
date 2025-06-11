use crate::repo;
pub fn branch_command(name: &str) {
    repo!().new_branch(name);
}
