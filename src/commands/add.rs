use crate::repo;
pub fn add_command(path: &str) {
    repo!().stage_file(path);
}
