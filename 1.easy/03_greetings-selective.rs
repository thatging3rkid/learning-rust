use std::io;

fn main() {
    println!("Please enter your name: "); // Print prompt
    let mut name = String::new(); // Make a String to store the input
    io::stdin().read_line(&mut name)
        .expect("Failed to read line!");

    match name.to_lowercase().trim().as_ref() {
        "alice" => println!("Hello, {}", name),
        "bob" => println!("Hello, {}", name),
        &_ => println!()
    }
}
