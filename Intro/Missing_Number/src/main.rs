use std::io;

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
        }
        Err(e) => println!("Oops! Something went wrong: {}", e),
    }
    input
}
fn main() {
    let array_size: i32 = read_input().trim().parse().unwrap();
    let array = read_input()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    
    let mut count: i32 = 0;
    let mut sum_X: i32 = 0;
    while count <= array_size {
        sum_X += count;
        count += 1;
    }

    let diff = sum_X - array.iter().sum::<i32>();
    println!("{}", diff);
}
