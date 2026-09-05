pub mod push{
    use std::process::exit;

use crate::obj::ref_storage::ref_storage;

    pub fn push_cmnd(args: Vec<String>){
        if args.is_empty() {
            eprintln!("Usage: dbit push <src_branch> <trgt_branch>");
            exit(-1);
        }

        let src = &args[0];
        let trgt = if args.len() > 1 {
            &args[1]
        } else {
            eprintln!("Usage: dbit push <src_branch> <trgt_branch>");
            exit(-2);
        };

        if !ref_storage::branch_exists(src) {
            println!("Src branch '{}' does not exist", src);
            exit(-3);
        }
        if !ref_storage::branch_exists(trgt) {
            println!("Trgt branch '{}' does not exist", trgt);
            exit(-4);
        }

        let src_hash = ref_storage::get_branch_hash(src);
        
        if let Some(hash) = src_hash {
            if !hash.is_empty() {
                ref_storage::set_branch_hash(trgt, &hash);
                println!("Pushed From: '{}' To: '{}'", src, trgt);
            } else {
                println!("Src '{}' has nothing to push", src);
            }
        } else {
            println!("Src '{}' has no commits", src);
        }
    }

}