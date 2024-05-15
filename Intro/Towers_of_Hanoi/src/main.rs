use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn hanoi(n: u16, start: u8, aux: u8, target:u8 ) {

    if n == 0 {
        return
    }
    hanoi(n-1, start, target, aux);
    println!("{} {}", start, target);
    hanoi(n-1, aux, start, target);
}

fn main() {
    let n: u16 = read_input().trim().parse().unwrap();
    println!("{}", 2u16.pow(n.clone().into()) - 1);
    hanoi(n, 1, 2, 3);
}
