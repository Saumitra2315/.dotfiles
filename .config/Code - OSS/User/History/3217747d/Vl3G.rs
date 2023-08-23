use std::{env, process::exit, path};
use std::fs;
fn run_file(path: &str) {
    let contents = fs:read_to_string(path);
}
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        println!("Usage: Bend[Script]");
        exit(64);
    }
    else if args.len() == 1 {
        run_file(args[1]);

    }else {
        run_prompt();
    }
    dbg!(args);
}