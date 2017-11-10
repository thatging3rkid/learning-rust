use std::io;
use std::io::Write;

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().ok()
        .expect("Problem flusing stdout"); // #justrustthings
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");
    
    let end: u32 = input.trim().parse().expect("Not a number!");

    let mut sum = 0;
    for x in 1..(end + 1) {
        if (x % 3) == 0 || (x % 5) == 0 {
            sum += x;
        }
    }

    println!("Sum of 1 to {}: {}", input.trim(), sum);
}
