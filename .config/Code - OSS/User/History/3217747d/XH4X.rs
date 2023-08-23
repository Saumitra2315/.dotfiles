use std::env;
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        println!("Usage: Bend[Script]")
    }
    dbg!(args);
}