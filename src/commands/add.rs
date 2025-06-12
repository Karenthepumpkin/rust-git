use crate::{debug_log, repo};
use std::fs;
use std::path::Path;
pub fn add_command(path: &str) {
    let p = Path::new(path);
    if p.is_file() {
        debug_log!("add {}", path);
        repo!().stage_file(path);
        // 处理文件
    } else if p.is_dir() {
        if p.file_name().map_or(false, |name| name == ".git") {
            debug_log!("skip .git dir");
            return;
        }
        debug_log!("add dir {} ", path);
        // 处理文件夹
        if let Ok(entries) = fs::read_dir(p) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    debug_log!("{} 是文件", path.display());
                    repo!().stage_file(path.to_str().unwrap());
                } else if path.is_dir() {
                    if let Some(s) = path.to_str() {
                        add_command(s);
                    } else {
                        debug_log!("Invalid UTF-8 path: {}", path.display());
                    }
                }
            }
        }
    } else {
        debug_log!("{} not a file or dir", path);
    }
}
