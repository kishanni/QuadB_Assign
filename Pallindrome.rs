use std::io;

fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<Vec<_>>();
    s.iter().eq(s.iter().rev())
}

fn main() {
    let mut input = String::new();
    
    println!("Enter a string to check if it is a palindrome:");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let input = input.trim(); 
    
    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
