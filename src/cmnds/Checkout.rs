pub mod checkout{
    use crate::obj::ref_storage::ref_storage;
    use crate::obj::commit::commit::get_commit;
    use crate::obj::tree::tree::get_tree;
    use crate::obj::blob::blob::get_blob;

    pub fn checkout_cmnd(args: Vec<String>){

        if args.is_empty() {
            println!("Usage: dbit checkout <branch>");
            return;
        }
        let branch = &args[0];
        if !ref_storage::branch_exists(branch) {
            println!("Branch '{}' does not exist", branch);
            return;
        }

        let branch_hash = ref_storage::get_branch_hash(branch);
        
        if let Some(hash) = branch_hash {
            if hash.is_empty() {
                println!("Branch '{}' has no commits", branch);
                ref_storage::set_head(&format!("refs/heads/{}", branch));
                println!("Switched to branch '{}'", branch);
                return;
            }

            if let Some(commit) = get_commit(&hash) {
                if let Some(tree) = get_tree(&commit.tree_hash) {
                    for ent in tree.entries {
                        if let Some(content) = get_blob(&ent.hash) {
                            std::fs::write(&ent.path, content).unwrap();
                            println!("Restored: {}", ent.path);
                        }
                    }
                }
            }       
            ref_storage::set_head(&format!("refs/heads/{}", branch));
            println!("Switched to '{}' and restored", branch);
        } else {
            ref_storage::set_head(&format!("refs/heads/{}", branch));
            println!("Switched to branch '{}'", branch);
        }
    }

}