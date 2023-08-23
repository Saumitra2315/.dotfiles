use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        println!("Usage: Bend[Script]")
    }
    else if args.len() == 1 {
        run_file(args[1]);

    }else {
        run_prompt();
    }
    dbg!(args);
}