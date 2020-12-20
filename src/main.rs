use rpassword::read_password;
use std::io::{stdout, stdin, Write};

fn main() {
    print!("Enter password: ");
    stdout().flush().unwrap();
    let password = read_password().unwrap();
    let password_chars = password.chars();

    println!();
    let mut input = String::new();
    println!("Enter the character numbers from the password, space separated: ");
    stdin().read_line(&mut input).unwrap();
    println!();

    let indices = input.split_whitespace();

    for (_index, item) in indices.enumerate() {
        let mut num: usize = item.parse().unwrap();
        num = num - 1;
        let char = password_chars.clone().nth(num).unwrap_or_default();
        println!("Char at {}: {}", num + 1, char);
    }
}
