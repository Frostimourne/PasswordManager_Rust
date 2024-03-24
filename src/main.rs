use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut password_vault: HashMap<String, String> = HashMap::new();

    loop {
        println!("Password Vault Menu:");
        println!("1. Add a new entry");
        println!("2. Retrieve an entry");
        println!("3. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => add_entry(&mut password_vault),
            2 => retrieve_entry(&password_vault),
            3 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_entry(password_vault: &mut HashMap<String, String>) {
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");

    password_vault.insert(username.trim().to_string(), password.trim().to_string());
    println!("Entry added successfully!");
}

fn retrieve_entry(password_vault: &HashMap<String, String>) {
    print!("Enter username to retrieve password: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");

    match password_vault.get(&username.trim().to_string()) {
        Some(password) => println!("Password: {}", password),
        None => println!("Entry not found for the given username."),
    }
}
