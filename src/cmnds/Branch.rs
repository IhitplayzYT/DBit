pub mod branch{
    use crate::fs_abs::Fs_basic::fs_basic::{dir_contains, mk_file};



    pub fn branch_cmnd(args: Vec<String>){
        if args.is_empty(){
            // Checkout available branchess
        
        }else{
            if args.len() >= 2{
                if matches!(&args[0][..],"-M"|"-m"){
                    let branch_name = &args[1];
                    // We check if branch exists or not, if not we make it
                    if dir_contains(branch_name, ".DBit/refs/heads"){
                       mk_file(branch_name,".DBit/refs");
                    }
                }
                else if matches!(&args[0][..],"-S"|"-s"|"-c"|"-C"){
                     // Write HEAD with conten ts of the branch
                }

            }



        }

    }

}