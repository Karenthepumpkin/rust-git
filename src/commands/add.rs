use crate::{debug_log, repo};
use std::fs;
use std::path::Path;
pub fn add_command(path: &str) {
    let p = Path::new(path);
    if p.is_file() {
        if let Some(name) = p.file_name().and_then(|os_str| os_str.to_str()) {
            if name == "rust-git" {
                debug_log!("skip rust-git");
                return;
            }
        }
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
                if let Some(path_str) = path.to_str() {
                    add_command(path_str);
                } else {
                    debug_log!("路径不是有效的 UTF-8 字符串");
                }
            }
        }
    } else {
        debug_log!("{} not a file or dir", path);
    }
}
