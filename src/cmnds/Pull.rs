pub mod pull{
    use std::process::exit;

use crate::obj::ref_storage::ref_storage;

    pub fn pull_cmnd(args: Vec<String>){

        if args.is_empty() {
            println!("Usage: dbit pull <src_branch> <trgt_branch>");
            exit(-9);
        }

        let src = &args[0];
        let trgt = if args.len() > 1 {
            &args[1]
        } else {
            println!("Usage: dbit pull <src_branch> <trgt_branch>");
            exit(-10);
        };

        if !ref_storage::branch_exists(src) {
            println!("Src branch '{}' does not exist", src);
            exit(-11);
        }

        if !ref_storage::branch_exists(trgt) {
            println!("Trgt branch '{}' does not exist", trgt);
            exit(-12);
        }

        let source_hash = ref_storage::get_branch_hash(src);

        if let Some(hash) = source_hash {
            if !hash.is_empty() {
                ref_storage::set_branch_hash(trgt, &hash);
                println!("Pulled From: '{}' To: '{}'", src, trgt);
            } else {
                println!("Src '{}' has nothing to pull", src);
            }
        } else {
            println!("Src '{}' has no commits", src);
        }
    }

}