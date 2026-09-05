pub mod diff{
    use std::path::Path;
    use crate::obj::ref_storage::ref_storage;
    use crate::obj::commit::commit::get_commit;
    use crate::obj::tree::tree::get_tree;
    use crate::obj::blob::blob::get_blob;
    use crate::obj::db_entities::db_entities::get_snapshot;

    pub fn diff_cmnd(args: Vec<String>){
        if args.is_empty() {
            println!("Usage: dbit diff --db <container_name>");
            return;
        }

        if args[0] == "--db" {
            let container_name = if args.len() > 1 {
                &args[1]
            } else {
                println!("Usage: dbit diff --db <container_name>");
                return;
            };

            let current_branch = ref_storage::get_current_branch();
            let branch_hash = ref_storage::get_branch_hash(&current_branch);
            if let Some(hash) = branch_hash {
                if !hash.is_empty() {
                    if let Some(commit) = get_commit(&hash) {
                        if let Some(tree) = get_tree(&commit.tree_hash) {
                            let snapshot_path = format!(".DBit/containers/{}.snapshot", container_name);
                            
                            for entry in tree.entries {
                                if entry.path == snapshot_path {
                                    if let Some(snapshot) = get_snapshot(&entry.hash) {
                                        println!("Database container: {}", snapshot.container_name);
                                        println!("Entities in commit:");
                                        for entity in &snapshot.entities {
                                            println!("  - {:?}: {}", entity.entity_type, entity.name);
                                        }
                                        return;
                                    }
                                }
                            }
                            println!("Container '{}' not found in last commit", container_name);
                            return;
                        }
                    }
                }
            }
            println!("No commits found for comparison");
            return;
        }
    }

}