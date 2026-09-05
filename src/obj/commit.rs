pub mod commit {
    use std::fs;
    use std::path::PathBuf;
    use serde::{Serialize, Deserialize};
    use sha2::{Sha256, Digest};


    #[derive(Serialize, Deserialize, Debug)]
    pub struct Commit {
        pub tree_hash: String,
        pub parent_hash: Option<String>,
        pub message: String,
        pub timestamp: i64,
        pub author: String,
    }

    pub fn create_commit(tree_hash: &str, parent_hash: Option<String>, message: &str, author: &str) -> String {
        let commit = Commit {tree_hash: tree_hash.to_string(),parent_hash,message: message.to_string(),timestamp: chrono::Utc::now().timestamp(),author: author.to_string()};
        let json = serde_json::to_string(&commit).unwrap();
        let salt: u64 = rand::random();
        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());
        hasher.update(salt.to_be_bytes());
        let hash = hex::encode(hasher.finalize());
        let obj_path = PathBuf::from(".DBit/objects").join(&hash);
        if !obj_path.exists() {
            fs::write(obj_path, json).unwrap();
        }
        hash
    }

    pub fn get_commit(hash: &str) -> Option<Commit> {
        let obj_path = PathBuf::from(".DBit/objects").join(hash);
        if obj_path.exists() {
            let content = fs::read_to_string(obj_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
}
