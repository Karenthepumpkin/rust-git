
use crate::core::repository::{self, Repository};
use crate::{debug_log, REPO};
use crate::repo;
pub fn commit_command(message: &str) {
	eprintln!("{}", repo!().commit(message));
}
