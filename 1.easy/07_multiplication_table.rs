use std::io;
use std::io::Write;

fn main() {
    print!("Enter the max number for the multplication table: ");

    io::stdout().flush().ok()
        .expect("Problem flusing stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let max: u32 = input.trim().parse().expect("Not a number!");
    
    let mut radix = 0;
    let mut max_val = max * max * 10;

    // Calculate the radix the slow way
    while max_val > 1 {
        max_val /= 10;
        radix += 1;
    }

    // Print preceding white space
    for _l in 1..(radix + 1) {
        print!(" ");
    }
    print!(" | ");

    
    for i in 0..(max + 1) {
        // Print the top row of the table
        if i == 0 {
            for j in 0..max {
                print!("{val:>3.3} | ", val = j.to_string());
            }
            print!("{val:>3.3}", val = max.to_string());
            println!();
        }
        // Print the actual numbers in each row
        for k in 0..(max + 1) {
            // Print the other column (the column that says what stuff if being multiplied by)
            if k == 0 {
                print!("{val:>3.3} | ", val = i.to_string());
            }

            // Print the value without a | if it's not at the end of the line
            if k != max {
                print!("{val:>3.3} | ", val = (i * k).to_string());
            } else {
                print!("{val:>3.3} ", val = (i * k).to_string());
            }
        }
        println!();
    }

}
