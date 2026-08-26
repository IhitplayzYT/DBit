pub mod fs_basic{
    use std::path::PathBuf;


    pub fn cwd_contains(file: &str) -> bool{
        for x in std::fs::read_dir(std::env::current_dir().unwrap()).unwrap(){
            let ent = x.unwrap();
            if ent.file_name() == file{
                return true;
            }
        }
        false
    }

    pub fn dir_contains(file: &str,dir: &str) -> bool{
        for x in std::fs::read_dir(dir).unwrap(){
            let ent = x.unwrap();
            if ent.file_name() == file{
                return true;
            }
        }
        false
    }

    pub fn mk_file(fname: &str,dir: &str){
        let fpath = PathBuf::from(dir).join(fname);
        std::fs::write(fpath,"PLACEHOLDER_HASH").unwrap();
    }



}