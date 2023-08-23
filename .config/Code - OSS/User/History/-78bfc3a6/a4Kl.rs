use clap::{Parser, command};
#[derive(Parser)]
#[command(name= "wet")]
#[command(about = "weather in your terminal")]
struct Args{
    days:u8,

}
fn main() {
    println!("Hello, world!");
}
