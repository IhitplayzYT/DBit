pub mod add{
    use std::{collections::{HashMap, HashSet}, path::PathBuf};
    use crate::obj::blob::blob;
    use crate::obj::ref_storage::ref_storage;

    pub fn recurse_paths(pth:&str,ret: &mut Vec<String>){
        let pth = PathBuf::from(pth);
        if !pth.exists(){
            return;
        }else if pth.is_dir(){
            if std::fs::read_dir(&pth).unwrap().next().is_none(){
                ret.push(pth.to_str().unwrap().to_string());
                return;
            }

            pth.read_dir().unwrap().for_each(|x|{
            let data = x.unwrap();
            if let Ok(p) = data.file_type(){
                if p.is_dir(){
                    recurse_paths(data.path().to_str().unwrap(), ret);
                }
                if p.is_file(){
                    ret.push(data.path().to_str().unwrap().to_string());
                }
            }
            });
        }else if pth.is_file(){
            ret.push(pth.to_str().unwrap().to_string());
        }
    }

    pub fn add_cmnd(args: Vec<String>,db_ignore: Option<Vec<String>>){
        let mut map:HashSet<String> = HashSet::new();
        
        if let Some(x) = db_ignore{
            x.into_iter().for_each(|k| {map.insert(k);});
        }
        let mut files: Vec<String> = vec![];
        for i in args{
            if &i == "."{
                recurse_paths(std::env::current_dir().unwrap().to_str().unwrap(), &mut files);
            }else{
                recurse_paths(&i,&mut files);
            }
        }
        
        let current_branch = ref_storage::get_current_branch();
        let mut staged_files: HashMap<String, String> = HashMap::new();
        
        for file in files {
            if !map.contains(&file) && !file.starts_with(".DBit") {
                if let Ok(hash) = std::panic::catch_unwind(|| {
                    blob::store_blob(&file, &current_branch)
                }) {
                    staged_files.insert(file.clone(), hash);
                }
            }
        }
        
        let staging_path = PathBuf::from(".DBit").join("staging");
        let serialized = serde_json::to_string(&staged_files).unwrap();
        std::fs::write(staging_path, serialized).unwrap();
    }

}