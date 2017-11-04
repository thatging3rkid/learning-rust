use std::io;

fn main() {
    println!("Please enter your name: "); // Print prompt
    let mut guess = String::new(); // Make a String to store the input
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    println!("Hello, {}", guess);
}
