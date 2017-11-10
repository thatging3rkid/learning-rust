use std::io;
use std::io::Write;

fn main() {
    // Get a number
    print!("Enter a number: ");
    io::stdout().flush().ok()
        .expect("Problem flushing stdout");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");

    // Figure out sum mode or multiply mode
    print!("Would you like to sum or multiply (s or m): ");
    io::stdout().flush().ok()
        .expect("Problem flushing stdout");
    
    let mut add_mode = String::new();
    io::stdin().read_line(&mut add_mode)
        .expect("Failed to read line!");

    // Figure out the mode
    let sum_mode;
    match add_mode.to_lowercase().trim().as_ref() {
        "s" => sum_mode = true,
        "m" => sum_mode = false,
        &_ => sum_mode = false
    }
    
    let end: u32 = input.trim().parse().expect("Not a number!");

    let mut sum = 0;
    for x in 1..(end + 1) {
        if sum_mode {
            sum += x;
        } else {
            if x == 1 {
                sum = 1;
            } else {
                sum *= x;
            }
        }
    }

    let out_str;
    if sum_mode {
        out_str = String::from("Sum");
    } else {
        out_str = String::from("Multiple");
    }
    
    println!("{} of 1 to {}: {}", out_str, input.trim(), sum);
}
