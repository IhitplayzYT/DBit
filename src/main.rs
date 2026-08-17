use crate::helper::Helper::CLI;

mod helper;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    println!("Hello, world!");
}
