use crate::debug_log;
use sha1::{Digest, Sha1};

pub fn hash(s: &Vec<u8>) -> [u8; 20] {
    // 使用 SHA-1 计算哈希
    let mut hasher = Sha1::new();
    hasher.update(s);
    let result = hasher.finalize();
    let hash: [u8; 20] = result.into();
    let result = String::from_utf8(s.clone());
    hash
}
pub fn hash2path(hash: [u8; 20]) -> String {
    let hash_str: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
    let (dir, file) = hash_str.split_at(2);
    format!(".git/objects/{}/{}", dir, file)
}
pub fn hashstr2path(hash_str: String) -> String {
    let (dir, file) = hash_str.split_at(2);
    format!(".git/objects/{}/{}", dir, file)
}
