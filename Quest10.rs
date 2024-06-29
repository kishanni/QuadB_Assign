use std::io;

fn Prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    
    println!("Enter a number to check if it's prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

   
    if Prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}
