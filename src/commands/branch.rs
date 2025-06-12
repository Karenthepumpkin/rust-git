use crate::repo;
pub enum BranchCommandType {
    New,
    Delete,
}
pub fn branch_command(name: &str, ctype: BranchCommandType) {
    match ctype {
        BranchCommandType::New => {
            repo!().new_branch(name);
        }
        BranchCommandType::Delete => {
            repo!().delete_branch(name);
        }
    }
}
