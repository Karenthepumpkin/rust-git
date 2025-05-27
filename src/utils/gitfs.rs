use std::path::Path;
use std::fs::File;
pub fn is_git_repo(path: &str) -> bool {
    let git_dir = format!("{}/.git", path);
    Path::new(&git_dir).exists() // 检测.git目录
}
pub enum FileType {
    BLOB,
    COMMIT,
    TREE,
}
pub fn add_git_file(filetype: FileType, content: &str) -> String {
    //TODO: Implement actual blob creation logic
    match filetype {
        FileType::BLOB => {
            let blob_string = format!("blob {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&blob_string);
            let path = crate::utils::hash::hash2path(hash);
            File::create(&path).expect("Failed to create blob file");
            path
        }
        FileType::COMMIT => {
            // 创建 COMMIT 对象
            let commit_string = format!("commit {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&commit_string);
            let path = crate::utils::hash::hash2path(hash);
            File::create(&path).expect("Failed to create blob file");
            path
        }
        FileType::TREE => {
            // 创建 TREE 对象
            let tree_string = format!("tree {}\0{}", content.len(), content);
            let hash = crate::utils::hash::hash(&tree_string);
            let path = crate::utils::hash::hash2path(hash);
            File::create(&path).expect("Failed to create blob file");
            path
        }
    }
}
