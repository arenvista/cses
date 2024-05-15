use std::io;

fn read_input() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_)  => {}
        Err(e) => println!("Something went wrong: {}",e)
    }
    input
}


fn main() {
    let input: i32 = read_input().trim().parse().unwrap();
    if input == 1{
        println!("1");
        return
    }
    let mut evens: Vec<i32> = Vec::new();
    let mut odds: Vec<i32> = Vec::new();
    for i in 1..input+1 {
        if i%2==0 {
            evens.push(i)
        }
        else {
            odds.push(i)
        }
    }
    evens.append(&mut odds);

    for i in 0..evens.len()-1{
        let diff = (evens[i] - evens[i+1]);
        if  diff == 1{
            println!("NO SOLUTION");
            return
        }
    }


    let output: String = evens.iter()
        .map(|&evens| evens.to_string() + " ")
        .collect();

    println!("{}", output);
    
}
