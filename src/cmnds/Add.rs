pub mod add{
    use std::{collections::{HashMap, HashSet}, fs::FileType, path::{Path, PathBuf}};


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

        // TODO: Further Processing here



    }

}