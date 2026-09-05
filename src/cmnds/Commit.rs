pub mod commit{
    use std::collections::HashMap;
    use std::path::PathBuf;
use std::process::exit;
    use crate::cmnds::Add::add::add_cmnd;
use crate::obj::tree::tree::{TreeEntry, create_tree};
    use crate::obj::commit::commit::{create_commit};
    use crate::obj::ref_storage::ref_storage;

    pub fn commit_cmnd(args: Vec<String>,db_ignore: Option<Vec<String>>){
        let mut message = if args.is_empty() {
            "Default message"
        } else {
            &args[0]
        };
        if matches!(&message.to_lowercase()[..],"-am"|"-ma") && args.len() > 1{
            add_cmnd(vec![".".to_string()], db_ignore);
            message = &args[1];
        }

        if matches!(message,"-m"|"-M") && args.len() > 1{
            message = &args[1];
        }

        if matches!(message,"-m"|"-M"|"-am"|"-ma"){
            eprintln!("Usage: dbit commit [-m|-M|-am|-ma] '<COMMIT_MESSAGE>'");
            exit(-5);
        }



        let staging_path = PathBuf::from(".DBit/staging");
        if !staging_path.exists() {
            println!("Nothing to commit");
            exit(-6);
        }


        let staged_content = std::fs::read_to_string(&staging_path).unwrap();
        let staged_files: HashMap<String, String> = serde_json::from_str(&staged_content).unwrap();
        if staged_files.is_empty() {
            println!("Nothing to commit");
            exit(-7);
        }

        let mut tree: Vec<TreeEntry> = Vec::new();
        for (path, hash) in staged_files {
            tree.push(TreeEntry {path,hash,mode: "100644".to_string()}); // Since blobs shouldn't be executable
        }
        let tree_hash = create_tree(tree);
        
        let curr = ref_storage::get_current_branch();
        let curr_hash = ref_storage::get_branch_hash(&curr);
        
        let author = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        let commit_hash = create_commit(&tree_hash, curr_hash, message, &author);
        
        ref_storage::set_branch_hash(&curr, &commit_hash);
        // FIXME:
        std::fs::write(&staging_path, "{}").unwrap();
        
        println!("Committed with hash: {}", commit_hash);
    }

}