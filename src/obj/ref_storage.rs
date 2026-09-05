pub mod ref_storage {
    use std::fs;
    use std::path::PathBuf;

    pub fn get_head() -> String {
        fs::read_to_string(".DBit/HEAD").unwrap_or("refs/heads/main".to_string())
    }

    pub fn set_head(pth: &str) {
        fs::write(".DBit/HEAD", pth).unwrap();
    }

    pub fn get_current_branch() -> String {
        let head = get_head();
        if head.starts_with("refs/heads/") {
            head.strip_prefix("refs/heads/").unwrap().to_string()
        } else {
            "main".to_string()
        }
    }

    pub fn get_branch_hash(branch: &str) -> Option<String> {
        let pth = PathBuf::from(".DBit/refs/heads").join(branch);
        if pth.exists() {
            Some(fs::read_to_string(pth).ok()?.trim().to_string())
        } else {
            None
        }
    }

    pub fn set_branch_hash(branch: &str, hash: &str) {
        let pth = PathBuf::from(".DBit/refs/heads").join(branch);
        fs::write(pth, hash).unwrap();
    }

    pub fn list_branches() -> Vec<String> {
        let mut ret = Vec::new();
        if let Ok(entries) = fs::read_dir(PathBuf::from(".DBit/refs/heads")) {
            for ent in entries.flatten() {
                if let Ok(x) = ent.file_name().into_string() {
                    ret.push(x);
                }
            }
        } 
        ret
    }

    pub fn branch_exists(branch: &str) -> bool {
        let pth = PathBuf::from(".DBit/refs/heads").join(branch);
        pth.exists()
    }

    pub fn create_branch(branch: &str) {
        let pth = PathBuf::from(".DBit/refs/heads").join(branch);
        fs::write(pth, "").unwrap();
    }
}
