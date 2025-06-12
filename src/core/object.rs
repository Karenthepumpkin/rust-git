use std::fs::File;
use std::path::Path;

use crate::debug_log;
use std::io::Write;
pub enum Object {
    Commit(String),
    Tree(String),
    Blob(Vec<u8>),
}
pub fn save(object: Object, workspace: &str) -> String {
    //TODO: Implement actual blob creation logic
    match object {
        Object::Blob(content) => {
            let blob_string = format!("blob {}\0", content.len());
            let mut blob_bytes = blob_string.into_bytes();
            blob_bytes.extend_from_slice(&content);
            let hash = crate::utils::hash::hash(&blob_bytes);
            let path = crate::utils::hash::hash2path(hash);
            let path = format!("{}/{}", workspace, path);
            if std::path::Path::new(&path).exists() {
                return hash.iter().map(|b| format!("{:02x}", b)).collect();
            }
            debug_log!("Creating blob file at: {}", path);
            std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap()).ok();
            let mut file = File::create(&path).expect("Failed to create blob file");
            file.write_all(&content)
                .expect("Failed to write blob content");
            hash.iter().map(|b| format!("{:02x}", b)).collect()
        }
        Object::Commit(content) => {
            // 创建 COMMIT 对象
            let commit_string = format!("commit {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&commit_string.as_bytes().to_vec());
            let path = crate::utils::hash::hash2path(hash);
            let path = format!("{}/{}", workspace, path);
            File::create(&path).expect(format!("Failed to create commit file{}", path).as_str());
            debug_log!("Creating commit file at: {}", path);
            let mut file = File::create(&path).expect("Failed to create commit file");
            file.write_all(content.as_bytes())
                .expect("Failed to write commit content");
            let hash_hex = hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            hash_hex
        }
        Object::Tree(content) => {
            // 创建 TREE 对象
            let tree_string = format!("tree {}\0{}", content.len(), content);
            let tree_bytes = tree_string.as_bytes().to_vec();
            let hash = crate::utils::hash::hash(&tree_bytes);
            let path = crate::utils::hash::hash2path(hash);
            let path = format!("{}/{}", workspace, path);
            File::create(&path).expect(format!("Failed to create tree file {}", path).as_str());
            debug_log!("Creating tree file at: {}", path);
            let mut file = File::create(&path).expect("Failed to create tree file");
            file.write_all(content.as_bytes())
                .expect("Failed to write tree content");
            let hash_hex = hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            hash_hex
        }
    }
}
