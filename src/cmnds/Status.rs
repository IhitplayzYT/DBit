pub mod status{
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::obj::ref_storage::ref_storage;
    use crate::obj::commit::commit::get_commit;

    pub fn status_cmnd(_args: Vec<String>){
        let curr = ref_storage::get_current_branch();
        println!("On branch {}", curr);
        
        let curr_hash = ref_storage::get_branch_hash(&curr);
        
        if let Some(hash) = curr_hash {
            if !hash.is_empty() {
                if let Some(commit) = get_commit(&hash) {
                    println!("({hash}) Last commit:  {}", commit.message);
                }
            } else {
                println!("No commits yet");
            }
        }
        
        let staging_path = PathBuf::from(".DBit/staging");
        if staging_path.exists() {
            let staged_content = std::fs::read_to_string(&staging_path).unwrap();
            let staged_files: HashMap<String, String> = serde_json::from_str(&staged_content).unwrap();
            if !staged_files.is_empty() {
                println!("\nStaged files:");
                for file in staged_files.keys() {
                    println!("  {}", file);
                }
            }
        }
    }

}