use std::io;

fn Checking_Prime(n: u32) -> bool {
    if n <= 1 {
        false
    } else {
        !(2..=(n as f64).sqrt() as u32).any(|i| n % i == 0)
    }
}

fn main() {
    
    println!("Enter a number to check prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

   
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please Enter a number to checked as prime!");
            return;
        }
    };

    
    if Checking_Prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}
