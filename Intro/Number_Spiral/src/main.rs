use std::io;

fn read_input() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => println!("Something went wrong: {}", e)
    }
    input
}

fn calc_diag(r: i128) -> i128 {
    return (r*r)-(r-1)
}

fn calc_bottom_half(r: i128, c: i128) -> i128 {
    let mut is_odd = false;

    if r % 2 == 0 {
        is_odd = false;
    } else {
        is_odd = true;
    }

    if is_odd == true {
        return calc_diag(r) + (c-r)
    } else {
        return calc_diag(r) - (c-r)
    }
}

fn calc_top_half(r: i128, c: i128) -> i128 {
    let mut is_odd = false;

    if r % 2 == 0 {
        is_odd = false;
    } else {
        is_odd = true;
    }

    if is_odd == true {
        return calc_diag(c) - (c-r)
    } else {
        return calc_diag(c) + (c-r)
    }
}

fn main() {
    let n_inputs: usize = read_input().trim().parse().unwrap();
    let mut answers: Vec<i128> = Vec::new();
    for _ in 0..n_inputs{
        let input: Vec<i128> = read_input().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let y: i128 = input[0];
        let x: i128 = input[1];
        if y > x {
            answers.push(calc_bottom_half(y, x))
        } else if y < x {
            answers.push(calc_top_half(y, x))
        } else {
            answers.push(calc_diag(y))
        }
    }
    for i in 0..answers.len() {
        println!("{}", answers[i]);
    }

}
