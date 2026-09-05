pub mod init{
    use std::fs;

use crate::fs_abs::Fs_basic::fs_basic::cwd_contains;

    fn create_dbit_dir(){
        fs::create_dir(".DBit").unwrap();
        std::env::set_current_dir(".DBit").unwrap();
        fs::write("HEAD","refs/heads/main").unwrap();
        fs::create_dir_all("refs/heads").unwrap();
        fs::write("refs/heads/main","").unwrap();
        fs::create_dir("objects").unwrap();
        std::env::set_current_dir("..").unwrap();
    }

    pub fn init_cmnd(_args: Vec<String>){
        if cwd_contains(".DBit"){
            panic!("The current Dir is already a DBit Repo");
        }
        create_dbit_dir();
    }

}