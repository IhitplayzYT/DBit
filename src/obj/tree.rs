pub mod tree {
    use std::fs;
    use std::path::PathBuf;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TreeEntry {
        pub path: String,
        pub hash: String,
        pub mode: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Tree {
        pub entries: Vec<TreeEntry>,
    }

    pub fn create_tree(entries: Vec<TreeEntry>) -> String {
        let tree = Tree { entries };
        let serialized = serde_json::to_string(&tree).unwrap();
        
        use rand::Rng;
        use sha2::{Sha256, Digest};
        
        let salt: u64 = rand::random();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        hasher.update(salt.to_be_bytes());
        
        let hash = hex::encode(hasher.finalize());
        let obj_path = PathBuf::from(".DBit/objects").join(&hash);
        
        if !obj_path.exists() {
            fs::write(obj_path, serialized).unwrap();
        }
        
        hash
    }

    pub fn get_tree(hash: &str) -> Option<Tree> {
        let obj_path = PathBuf::from(".DBit/objects").join(hash);
        if obj_path.exists() {
            let content = fs::read_to_string(obj_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
}
