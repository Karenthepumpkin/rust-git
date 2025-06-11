use std::fs::File;
use std::path::Path;

use crate::debug_log;
use std::io::Write;
pub enum Object {
    Commit(String),
    Tree(String),
    Blob(String),
}
pub fn save(object: Object, workspace: &str) -> String {
    //TODO: Implement actual blob creation logic
    match object {
        Object::Blob(content) => {
            let blob_string = format!("blob {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&blob_string);
            let path = crate::utils::hash::hash2path(hash);
            let path = format!("{}/{}", workspace, path);
            debug_log!("Creating blob file at: {}", path);
            File::create(&path).expect(format!("Failed to create blob file {}", path).as_str());
            let mut file = File::create(&path).expect("Failed to create blob file");
            file.write_all(content.as_bytes())
                .expect("Failed to write blob content");
            path
        }
        Object::Commit(content) => {
            // 创建 COMMIT 对象
            let commit_string = format!("commit {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&commit_string);
            let path = crate::utils::hash::hash2path(hash);
            let path = format!("{}/{}", workspace, path);
            File::create(&path).expect(format!("Failed to create commit file{}", path).as_str());
            debug_log!("Creating commit file at: {}", path);
            let mut file = File::create(&path).expect("Failed to create commit file");
            file.write_all(content.as_bytes())
                .expect("Failed to write commit content");
            path
        }
        Object::Tree(content) => {
            // 创建 TREE 对象
            let tree_string = format!("tree {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&tree_string);
            let path = crate::utils::hash::hash2path(hash);
            let path = format!("{}/{}", workspace, path);
            File::create(&path).expect(format!("Failed to create tree file {}", path).as_str());
            debug_log!("Creating tree file at: {}", path);
            let mut file = File::create(&path).expect("Failed to create tree file");
            file.write_all(content.as_bytes())
                .expect("Failed to write tree content");
            path
        }
    }
}
