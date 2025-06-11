
use crate::core::repository::{self, Repository};
use crate::{debug_log, REPO};
use crate::repo;
pub fn rm_command(path: &str) {
	repo!().unstage_file(path);
	if let Err(e) = std::fs::remove_file(path) {
		eprintln!("Failed to remove file '{}': {}", path, e);
	}
}
