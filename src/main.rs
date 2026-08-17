use std::fs;

use crate::{cmnds::{Add::add::add_cmnd, Branch::branch::branch_cmnd, Checkout::checkout::checkout_cmnd, Commit::commit::commit_cmnd, Diff::diff::diff_cmnd, Init::init::init_cmnd, Pull::pull::pull_cmnd, Push::push::push_cmnd, Status::status::status_cmnd}, helper::Helper::{CLI, Cmnd}};

mod helper;
mod cmnds;
fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    let db_ignore:Option<Vec<String>> = match fs::read_to_string(".DBignore"){
        Ok(x) => {Some(x.split("\n").map(|y| y.trim().to_string()).collect())},
        Err(_) => {None}
    };


    match clargs.cmnd{
        Cmnd::Init => {init_cmnd(clargs.args)},
        Cmnd::Add => {add_cmnd(clargs.args,db_ignore)},
        Cmnd::Commit => {commit_cmnd(clargs.args)},
        Cmnd::Diff => {diff_cmnd(clargs.args)},
        Cmnd::Push => {push_cmnd(clargs.args)},
        Cmnd::Pull => {pull_cmnd(clargs.args)},
        Cmnd::Checkout => {checkout_cmnd(clargs.args)},
        Cmnd::Branch => {branch_cmnd(clargs.args)},
        Cmnd::Status => {status_cmnd(clargs.args)}
    }

}
