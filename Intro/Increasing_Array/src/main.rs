use std::io;
fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => { }
        Err(e) => println!("Oops! Something went wrong: {}", e)
    }
    input
}

fn main() {
    let mut total_moves: usize = 0;

    let mountain_size: usize = read_input().trim().parse().unwrap();
    let mut mountain = read_input()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for landmark_index in 0..(mountain_size-1) {
        while mountain[landmark_index] > mountain[landmark_index+1]{
            let difference: usize = (mountain[landmark_index] - mountain[landmark_index+1]) as usize;
            mountain[landmark_index+1] = mountain[landmark_index];
            total_moves += difference;
        }
    }
    println!("{}",total_moves);
}
