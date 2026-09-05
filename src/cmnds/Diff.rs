pub mod diff{
    use std::path::Path;
use std::process::exit;
    use crate::obj::ref_storage::ref_storage;
    use crate::obj::commit::commit::get_commit;
    use crate::obj::tree::tree::get_tree;
    use crate::obj::blob::blob::get_blob;

    pub fn diff_cmnd(args: Vec<String>){
        let file_path = if args.is_empty() {
            println!("Usage: dbit diff <fpath>");
            return;
        } else {
            &args[0]
        };

        if !Path::new(file_path).exists() {
            println!("File: '{}' does not exist", file_path);
            exit(-8);
        }

        let curr = ref_storage::get_current_branch();
        let curr_hash = ref_storage::get_branch_hash(&curr);        
        let curr_content = std::fs::read_to_string(file_path).unwrap_or_default();

        
        if let Some(hash) = curr_hash {
            if !hash.is_empty() {
                if let Some(commit) = get_commit(&hash) {
                    if let Some(tree) = get_tree(&commit.tree_hash) {
                        for entry in tree.entries {
                            if entry.path == *file_path {
                                if let Some(old_content_bytes) = get_blob(&entry.hash) {
                                    let old_content = String::from_utf8_lossy(&old_content_bytes).to_string();
                                    
                                    if old_content == curr_content {
                                        println!("Unchanged {}", file_path);
                                    } else {
                                        println!("Differences in {}:", file_path);

                                        let old_lines: Vec<&str> = old_content.lines().collect();
                                        let new_lines: Vec<&str> = curr_content.lines().collect();
                                        
                                        for (i, (old, new)) in old_lines.iter().zip(new_lines.iter()).enumerate() {
                                            if old != new {
                                                println!("-- Line {}: {}", i + 1, old);
                                                println!("++ Line {}: {}", i + 1, new);
                                        }

                                        // Remaining Extra lines
                                        if old_lines.len() > new_lines.len() {
                                            for i in new_lines.len()..old_lines.len() {
                                                println!("-- Line {}: {}", i + 1, old_lines[i]);
                                            }
                                        } else if new_lines.len() > old_lines.len() {
                                            for i in old_lines.len()..new_lines.len() {
                                                println!("++ Line {}: {}", i + 1, new_lines[i]);
                                            }
                                        }
                                    }
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        println!("New File '{}' ", file_path);
    }

}

}