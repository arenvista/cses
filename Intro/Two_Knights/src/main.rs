use std::io;

fn calculate_total_positions(k: i64) -> i64 {
    k.pow(2) * (k.pow(2) - 1) / 2
}

fn calculate_engaged_positions(k: i64) -> i64 {
    4 * (k-1) * (k-2)
}

fn calculat_valid_positions(k: i64) -> i64 {
    if k == 1 {
        return 0;
    } else if k == 2 {
        return 6;
    } else {
        return calculate_total_positions(k) - calculate_engaged_positions(k);
    }

}

fn read_input() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => println!("Something went wrong: {}", e)
    }
    input
}

fn main() {
    let input: i64 = read_input().trim().parse().unwrap();
    for i in 1..input+1 {
        println!("{}", calculat_valid_positions(i));
    }
}
