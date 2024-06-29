use std::io;

fn common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}

fn main() {
    // Read input strings from user
    println!("Enter strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input into a vector of strings
    let strs: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    // Find the longest common prefix and print the result
    let prefix = common_prefix(&strs);
    if prefix.is_empty() {
        println!("There is no common prefix.");
    } else {
        println!("The longest common prefix is '{}'", prefix);
    }
}
