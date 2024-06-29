use std::io::{self, BufRead};

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

fn main() {
   
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).expect("Failed to read input");

 
    let input = input.trim();

    
    match shortest_word(input) {
        Some(shortest) => println!("The shortest word is in the given string is '{}'", shortest),
        None => println!("No words found in the given string"),
    }
}
