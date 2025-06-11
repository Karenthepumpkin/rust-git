use crate::core::repository::{self, Repository};
use crate::{debug_log, REPO};
use crate::repo;
pub fn add_command(path: &str) {
	repo!().stage_file(path);
}
