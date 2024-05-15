use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

//TODO: reneme this function
fn do_something (coin_pair: Vec<u32> ) -> String {
    //println!("{:?}", coin_pair);
    let total: u32 = coin_pair[0] + coin_pair[1];
    if total%3 != 0 || coin_pair[0] > 2 * coin_pair[1] || coin_pair[1] > 2 * coin_pair[0]{
        return "NO".to_string()
    } else {
        return "YES".to_string()
    }
}
fn main() {
    let n_entries: u32 = read_input().trim().parse().unwrap();
    let mut output: Vec<String> = Vec::new();

    for _ in 1..=n_entries{
        let coin_pair:Vec<u32> = read_input()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse #"))
            .collect();
        output.push(do_something(coin_pair));
    }
    for o in output{
        println!("{}", o)
    }
}
