pub mod blob {
    use rand::Rng;
    use sha2::{Sha256, Digest};
    use std::fs;
    use std::path::PathBuf;

    pub fn compute_hash(fpath: &str, branch: &str) -> String {
        let data = fs::read(fpath).unwrap();
        let l = data.len();
        let salt: u64 = rand::random();        
        let mut hasher = Sha256::new();
        hasher.update(l.to_be_bytes());
        hasher.update(branch.as_bytes());
        hasher.update(salt.to_be_bytes());
        hasher.update(&data);
        hex::encode(hasher.finalize())
    }

    pub fn store_blob(fpath: &str, branch: &str) -> String {
        let hash = compute_hash(fpath, branch);
        let obj_path = PathBuf::from(".DBit/objects").join(&hash);        
        if !obj_path.exists() {
            let content = fs::read(fpath).unwrap();
            fs::write(obj_path, content).unwrap();
        }
        hash
    }

    pub fn stash_blob(fpath: &str,branch: &str) -> String{
        store_blob(fpath, branch)
    }

    pub fn get_blob(hash: &str) -> Option<Vec<u8>> {
        let obj_path = PathBuf::from(".DBit/objects").join(hash);
        if obj_path.exists() {
            fs::read(obj_path).ok()
        } else {
            None
        }
    }
}
