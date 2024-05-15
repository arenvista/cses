use std::io;
use std::collections::HashSet;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.pop();
    input
}

fn generate_strings(original_string: Vec<char>) -> (usize, HashSet<String>) {
    let mut permutations = HashSet::from([original_string.clone().into_iter().collect()]);
    for (index_1, character) in original_string.clone().iter().enumerate() {
        for (index_2, next_character) in original_string.clone().iter().enumerate() {
            if character != next_character {
                let mut new_string = original_string.clone();
                new_string[index_1] = *next_character;
                new_string[index_2] = *character;
                permutations.insert(new_string.into_iter().collect());
            }
    }
    }

    return (permutations.len(), permutations);
}


fn main() {
    let input: Vec<char> = read_input().chars().collect();
    let permutations = generate_strings(input);
    println!("{}", permutations.0);
    for string in permutations.1 {
        println!("{}", string);
    }
}
