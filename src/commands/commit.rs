use crate::core::repository::{self, Repository};
use crate::repo;
use crate::{REPO, debug_log};
pub fn commit_command(message: &str) {
    println!("{}", repo!().commit(message));
}
