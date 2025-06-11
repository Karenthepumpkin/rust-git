use sha1::{Digest, Sha1};

pub fn hash(s: &str) -> [u8; 20] {
    // 使用 SHA-1 计算哈希
    let mut hasher = Sha1::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    let hash: [u8; 20] = result.into();
    hash
}
pub fn hash2path(hash: [u8; 20]) -> String {
    let hash_str: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
    let (dir, file) = hash_str.split_at(2);
    format!(".git/objects/{}/{}", dir, file)
}
