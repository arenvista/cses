use std::io;
fn weird(input: i64) -> i64 {
    if input % 2 == 0 {
        return input / 2;
    } else {
        return 3 * input + 1;
    }
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
        }
        Err(error) => println!("error: {error}"),
    }

    let mut input: i64 = input.trim().parse().unwrap();
    let mut owned_string: String = "".to_owned();
    owned_string.push_str(&input.to_string());
    owned_string.push_str(" ");
    while input != 1 {
        input = weird(input);
        owned_string.push_str(&input.to_string());
        owned_string.push_str(" ");
    }

    println!("{}", owned_string);
}
