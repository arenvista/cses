use std::io;

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {}
        Err(e) => println!("Some error occurred {}", e)
    }
    input
}


fn main() {
    const MOD: u64 = 1_000_000_007; // 10 ^ 9 + 7
    let n: u128 = read_input().trim().parse().unwrap();
    let res = (0..n).fold(1, |res, _| 2 * res % MOD);
    println!("{res}");

}
