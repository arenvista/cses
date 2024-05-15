use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input
}

fn main() {
    let n: usize = read_input().trim().parse().unwrap();
    (0..(0b1 << n)).for_each(|b| println!("{:<01$b}", (b >> 0b1) ^ b, n))
}
