use aoclib::read_to_chars;
fn main(){
    let ch = read_to_chars("input/2015-01.txt");
    println!(
        ch.iter().map(|x| match  x {
            '(' => 1,
            ')' => -1
            _ => panic!("invalid character input")
        }).sum::<i32>()
);z
}