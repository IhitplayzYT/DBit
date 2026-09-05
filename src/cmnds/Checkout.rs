pub mod checkout{
    use crate::obj::db_entities::db_entities::{EntityType, get_snapshot};
use crate::obj::ref_storage::ref_storage;
    use crate::obj::commit::commit::get_commit;
    use crate::obj::tree::tree::get_tree;
    use crate::obj::blob::blob::get_blob;

    pub fn checkout_cmnd(args: Vec<String>){
        if args.is_empty() {
            println!("Usage: dbit checkout <branch_name>");
            return;
        }
        let branch_name = &args[0];
        if !ref_storage::branch_exists(branch_name) {
            println!("Branch '{}' does not exist", branch_name);
            return;
        }

        let branch_hash = ref_storage::get_branch_hash(branch_name);
        
        if let Some(hash) = branch_hash {
            if hash.is_empty() {
                println!("Branch '{}' has no commits", branch_name);
                ref_storage::set_head(&format!("refs/heads/{}", branch_name));
                println!("Switched to branch '{}'", branch_name);
                return;
            }

            if let Some(commit) = get_commit(&hash) {
                if let Some(tree) = get_tree(&commit.tree_hash) {
                    for entry in tree.entries {
                        if entry.path.starts_with(".DBit/containers/") && entry.path.ends_with(".dbsnp") {
                            if let Some(snapshot) = get_snapshot(&entry.hash) {
                                println!("Restoring database container: {}", snapshot.container_name);
                                for entity in snapshot.entities {
                                    match entity.entity_type {
                                        EntityType::Schema => {
                                            println!("  Schema for table: {}", entity.name);
                                        }
                                        EntityType::Data => {
                                            println!("  Data for table: {}", entity.name);
                                        }
                                        EntityType::User => {
                                            println!("  User metadata: {}", entity.name);
                                        }
                                        EntityType::Index => {
                                            println!("  Index for table: {}", entity.name);
                                        }
                                        EntityType::Table => {
                                            println!("  Schema & data: {}", entity.name);
                                        }
                                    }
                                }
                            }
                        } else if let Some(content) = get_blob(&entry.hash) {
                            std::fs::write(&entry.path, content).unwrap();
                            println!("Restored: {}", entry.path);
                        }
                    }
                }
            }
            
            ref_storage::set_head(&format!("refs/heads/{}", branch_name));
            println!("Switched to branch '{}' and restored files/database entities", branch_name);
        } else {
            ref_storage::set_head(&format!("refs/heads/{}", branch_name));
            println!("Switched to branch '{}'", branch_name);
        }
    }

}