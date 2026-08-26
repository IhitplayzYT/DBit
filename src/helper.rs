pub mod Helper{
    use std::process::exit;



    const DBG_STR: &str = "";
    const OK:i32 = 0;
    const ERR:i32 = -1;

    #[derive(Debug,Clone)]
    pub enum Cmnd{
        Init,
        Add,
        Commit,
        Diff,
        Push,
        Pull,
        Checkout,
        Branch,
        Status
    }


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub cmnd: Cmnd,
        pub args: Vec<String>,
        
    }

    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false,cmnd:Cmnd::Init,args:vec![]}
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().collect();
            if args.len() >= 1{
                self.cmnd = match &args[0][..]{
                    "init" => Cmnd::Init,
                    "add" => Cmnd::Add,
                    "commit" => Cmnd::Commit,
                    "diff" => Cmnd::Diff,
                    "push" => Cmnd::Push,
                    "pull" => Cmnd::Pull,
                    "checkout" => Cmnd::Checkout,
                    "branch" => Cmnd::Branch,
                    "status" => Cmnd::Status,
                    _ => {panic!("Invalid Command");}
                }
            }else{
                panic!("Need atleast one CLI arg");
            }

           for i in args.iter().skip(1){
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                } else{
                    self.args.push(i.to_string());
                }
           } 

        }



    }


    





}