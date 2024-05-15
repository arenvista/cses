use std::io;
use std::collections::HashMap;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input
}

fn count_chars(input: Vec<char>) -> HashMap<char, u32> {
    let mut char_count = HashMap::new();
    for c in input {
        match char_count.get(&c) {
            Some(count) => char_count.insert(c, count + 1),
            None => char_count.insert(c, 1),
        };
    }
        char_count
}

fn validate_map(char_count: &HashMap<char, u32>) -> bool {
    let mut odd_count = 0;
    for (_, count) in char_count {
        if count % 2 != 0 {
            odd_count += 1;
        } 
        if odd_count > 1 {
            return false;
        }
    }
    return true;
}

fn print_palindrone(char_count: HashMap<char, u32>) {
    let mut palindrone: String = String::new();
    for (c, count) in &char_count {
        if count % 2 != 0 {
            let pal_center = palindrone.len()/2;
            let str_insrt: String = c.to_string().repeat((*count) as usize);
            palindrone.insert_str(pal_center,&str_insrt);
        } else {
        let str_insrt: String = c.to_string().repeat((*count/2) as usize);

        palindrone = str_insrt.to_owned() + &palindrone + &str_insrt.to_owned();
        }
    }
    println!("{}", palindrone);
}

fn main(){
    let input: Vec<char> = read_input().trim().chars().collect();
    let char_count = count_chars(input);
    match validate_map(&char_count) {
        true => {},
        false =>{ 
            println!("NO SOLUTION");
            return
        }
    }
    print_palindrone(char_count);
}
