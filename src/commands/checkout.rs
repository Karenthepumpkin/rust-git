use crate::repo;
pub fn checkout_command(name: &str) {
    repo!().set_current_branch(name);
}
