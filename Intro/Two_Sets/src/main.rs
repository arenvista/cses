use std::io;

fn generate_sequence(n:u32) -> Vec<u32> {
    (1..n+1).collect()
}

fn read_input() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {}
        Err(e) => println!("Some err {}", e)
    }
    input
    }

fn test_validity(in_seq: &Vec<u32>) -> bool {
    let sum: u32 = in_seq.iter().sum();
    if sum%2 == 0{
        println!("YES");
        return true
    } else {
        println!("NO");
        return false
    }
}

fn create_sets(input: u32, in_seq: Vec<u32>) -> Vec< Vec<u32> > {
    let mut set1: Vec<u32> = Vec::new();
    let mut set2: Vec<u32> = Vec::new();
    let is_even: bool = input%2 == 0;
    if is_even == true {
        let mut flip: bool = false;
        for i in &in_seq{
            if flip == false && *i<(&input/2) {
                set1.push(*i);
                let target = in_seq[(&input-i) as usize];
                set1.push(target);
                flip = true
            } else  if *i<=(&input/2){
                set2.push(*i);
                let target = in_seq[(&input-i) as usize];
                set2.push(target);
                flip = false
            }
        }
    } else {
        set2.push(in_seq[(input-1) as usize]);
        let mut flip: bool = false;
        for i in &in_seq[0..input as usize]{
            if flip == false && *i<=(&input/2){
                set1.push(*i);
                let target = in_seq[0..input as usize][(&input-i-1) as usize];
                set1.push(target);
                flip = true
            } else  if *i<(&input/2){
                set2.push(*i);
                let target = in_seq[0..input as usize][(&input-i-1) as usize];
                set2.push(target);
                flip = false
            }
        }
    }
    let output: Vec<Vec<u32>> = vec![set1, set2];
    return output
}

fn main() {
    let input: u32 = read_input().trim().parse().unwrap();

    let in_seq: Vec<u32> = generate_sequence(input);

    match test_validity(&in_seq) {
        true => {},
        false => return,
    };

    let sets:Vec<Vec<u32>> = create_sets(input, in_seq);
    for vec in &sets{
        let part_set = vec.iter().map(|val| format!("{}", val)).collect::<Vec<String>>().join(" ");
        println!("{}", vec.len());
        println!("{}", part_set);
    }
}
