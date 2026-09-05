pub mod branch{
    use crate::obj::ref_storage::ref_storage;

    pub fn branch_cmnd(args: Vec<String>){
        if args.is_empty(){
            let all_brnch = ref_storage::list_branches();
            let curr = ref_storage::get_current_branch();
            
            println!("Branches: ");
            for branch in all_brnch {
                if branch == curr {
                    println!("  {} <- Current", branch);
                } else {
                    println!("  {}", branch);
                }
            }
        } else {
            let branch = &args[0];
            if args.len() >= 2 {
                if matches!(&args[0][..],"-M"|"-m") {
                    let n_name = &args[1];
                    if ref_storage::branch_exists(n_name) {
                        println!("Branch '{}' exists", n_name);
                    } else {
                        let curr = ref_storage::get_current_branch();
                        let curr_hash = ref_storage::get_branch_hash(&curr);
                        ref_storage::create_branch(n_name);
                        if let Some(hash) = curr_hash {
                            ref_storage::set_branch_hash(n_name, &hash);
                        }
                        println!("Branch '{}' created", n_name);
                    }
                }
                else if matches!(&args[0][..],"-S"|"-s"|"-c"|"-C") {
                    let trgt = &args[1];
                    if ref_storage::branch_exists(trgt) {
                        ref_storage::set_head(&format!("refs/heads/{}", trgt));
                        println!("Switched to '{}' branch", trgt);
                    } else {
                        println!("Branch '{}' does not exist", trgt);
                    }
                }
            } else {
                if ref_storage::branch_exists(branch) {
                    println!("Branch '{}' exists", branch);
                } else {
                    let curr = ref_storage::get_current_branch();
                    let curr_hash = ref_storage::get_branch_hash(&curr);
                    ref_storage::create_branch(branch);
                    if let Some(hash) = curr_hash {
                        ref_storage::set_branch_hash(branch, &hash);
                    }
                    println!("Branch '{}' created", branch);
                }
            }
        }
    }

}