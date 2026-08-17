pub mod init{
    use std::fs;

    fn create_dbit_dir(){
        fs::create_dir(".DBit").unwrap();
        std::env::set_current_dir(".DBit").unwrap();
        fs::write("PATH","refs/head").unwrap();
        fs::create_dir("refs").unwrap();
        fs::create_dir("objects").unwrap();
        std::env::set_current_dir("..").unwrap();
    }





    pub fn init_cmnd(_args: Vec<String>){
        create_dbit_dir();
    }

}