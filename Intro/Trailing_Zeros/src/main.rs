use std::io;

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => println!("Some Error {}", e)
    }
    input
}

fn main() {
    let mut input: u32 = read_input().trim().parse().unwrap();
    let mut count: u32 = 0;
    while input > 0 {
        count += input / 5;
        input /= 5;
    }
    println!("{}", count);
}
