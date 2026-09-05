pub mod commit{
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::obj::tree::tree::{TreeEntry, create_tree};
    use crate::obj::commit::commit::{create_commit};
    use crate::obj::ref_storage::ref_storage;
    use crate::obj::db_entities::db_entities::{DBSnapshot, store_snapshot};

    pub fn commit_cmnd(args: Vec<String>){
        let message = if args.is_empty() {
            "Default commit message"
        } else {
            &args[0]
        };

        let staging_path = PathBuf::from(".DBit/staging");
        let staging_db_path = PathBuf::from(".DBit/staging_db");
        
        let has_files = staging_path.exists();
        let has_db_entities = staging_db_path.exists();

        if !has_files && !has_db_entities {
            println!("Nothing to commit. Use 'dbit add' to stage files or database entities.");
            return;
        }

        let mut tree_entries: Vec<TreeEntry> = Vec::new();
        let mut db_snapshots: Vec<String> = Vec::new();

        if has_files {
            let staged_content = std::fs::read_to_string(&staging_path).unwrap();
            let staged_files: HashMap<String, String> = serde_json::from_str(&staged_content).unwrap();

            if !staged_files.is_empty() {
                for (path, hash) in staged_files {
                    tree_entries.push(TreeEntry {
                        path,
                        hash,
                        mode: "100644".to_string(),
                    });
                }
            }
        }

        if has_db_entities {
            let staged_content = std::fs::read_to_string(&staging_db_path).unwrap();
            let staged_entities: Vec<crate::obj::db_entities::db_entities::DBEntity> = serde_json::from_str(&staged_content).unwrap();

            if !staged_entities.is_empty() {
                let mut container_entities: HashMap<String, Vec<crate::obj::db_entities::db_entities::DBEntity>> = HashMap::new();
                
                for entity in staged_entities {
                    container_entities.entry(entity.container_name.clone())
                        .or_insert_with(Vec::new)
                        .push(entity);
                }

                for (container_name, entities) in container_entities {
                    let snapshot = DBSnapshot {
                        container_name: container_name.clone(),
                        entities,
                    };
                    let snapshot_hash = store_snapshot(&snapshot);
                    db_snapshots.push(snapshot_hash.clone());
                    
                    tree_entries.push(TreeEntry {
                        path: format!(".DBit/containers/{}.dbsnp", container_name),
                        hash: snapshot_hash,
                        mode: "100644".to_string(),
                    });
                }
            }
        }

        if tree_entries.is_empty() {
            println!("Nothing to commit.");
            return;
        }

        let tree_hash = create_tree(tree_entries);
        
        let current_branch = ref_storage::get_current_branch();
        let parent_hash = ref_storage::get_branch_hash(&current_branch);
        
        let author = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        let commit_hash = create_commit(&tree_hash, parent_hash, message, &author);
        
        ref_storage::set_branch_hash(&current_branch, &commit_hash);
        
        if has_files {
            std::fs::write(&staging_path, "{}").unwrap();
        }
        if has_db_entities {
            std::fs::write(&staging_db_path, "{}").unwrap();
        }
        
        println!("Committed: {}", commit_hash);
        println!("Tree: {}", tree_hash);
        if !db_snapshots.is_empty() {
            println!("Database snapshots: {}", db_snapshots.len());
        }
    }

}