// 核心对象类型枚举
/* 
pub enum Object {
    Commit(String), // 提交对象：存储提交信息
    Tree(String),
    // 树对象：存储目录结构
    Blob(String),
}
impl Object {
    /// 对象存储算法
    // 数据对象：存储文件内容
    pub fn save(&self, repo_path: &str) -> String {
        // 1. 序列化对象数据
        let raw_data = match self {
            Object::Commit(data) => format!("commit {}\0{}", data.len(), data),
            Object::Tree(data) => format!("tree {}\0{}", data.len(), data),
            Object::Blob(data) => format!("blob {}\0{}", data.len(), data),
        };
        let hash = sha1(raw_data);
        let dir = format!("{}/.git/objects/{}", repo_path, &hash[0..2]);
        create_dir(dir);
        write_file(format!("{}/{}", dir, &hash[2..]), raw_data);
        hash // 返回对象哈希
    }
}
*/