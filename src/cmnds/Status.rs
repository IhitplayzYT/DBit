pub mod status{
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::obj::ref_storage::ref_storage;
    use crate::obj::commit::commit::get_commit;

    pub fn status_cmnd(_args: Vec<String>){
        let current_branch = ref_storage::get_current_branch();
        println!("On branch {}", current_branch);
        
        let branch_hash = ref_storage::get_branch_hash(&current_branch);
        
        if let Some(hash) = branch_hash {
            if !hash.is_empty() {
                if let Some(commit) = get_commit(&hash) {
                    println!("Last commit: {}", commit.message);
                    println!("Commit hash: {}", hash);
                }
            } else {
                println!("No commits yet");
            }
        }
        
        let staging_path = PathBuf::from(".DBit/staging");
        let staging_db_path = PathBuf::from(".DBit/staging_db");
        
        let mut has_staged = false;

        if staging_path.exists() {
            let staged_content = std::fs::read_to_string(&staging_path).unwrap();
            let staged_files: HashMap<String, String> = serde_json::from_str(&staged_content).unwrap();
            
            if !staged_files.is_empty() {
                println!("\nStaged files:");
                for file in staged_files.keys() {
                    println!("  {}", file);
                }
                has_staged = true;
            }
        }

        if staging_db_path.exists() {
            let staged_content = std::fs::read_to_string(&staging_db_path).unwrap();
            let staged_entities: Vec<crate::obj::db_entities::db_entities::DBEntity> = serde_json::from_str(&staged_content).unwrap();
            
            if !staged_entities.is_empty() {
                println!("\nStaged database entities:");
                let mut container_counts: HashMap<String, usize> = HashMap::new();
                for entity in &staged_entities {
                    *container_counts.entry(entity.container_name.clone()).or_insert(0) += 1;
                }
                for (container, count) in container_counts {
                    println!("  Container '{}': {} entities", container, count);
                }
                has_staged = true;
            }
        }
        
        if !has_staged {
            println!("\nNothing staged");
        }
        
        println!("\nWorking directory clean");
    }

}