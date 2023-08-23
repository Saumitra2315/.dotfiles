use aoclib::read_to_chars;
fn main(){
    let ch = read_to_chars("input/2015-01.txt");
    println!(
        "day 1 part 1 = {}",
        ch.iter().map(|x| match  x {
            '(' => 1,
            ')' => -1,
            _ => panic!("invalid character input")
        }).sum::<i32>()
);
}