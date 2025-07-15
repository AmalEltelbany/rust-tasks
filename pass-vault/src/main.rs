mod pentry;

use std::ascii;
use std::io::Error;

use crate::pentry::prompt;

use crate::pentry::ServiceInfo;
use crate::pentry::read_passwords_from_file;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
     █████╗ ███╗   ███╗ █████╗ ██╗     ██    ██  █████╗ ██╗   ██╗██╗     
    ██╔══██╗████╗ ████║██╔══██╗██║     ██    ██ ██╔══██╗██║   ██║██║     
    ███████║██╔████╔██║███████║██║     ██    ██ ███████║██║   ██║██║     
    ██╔══██║██║╚██╔╝██║██╔══██║██║     ██    ██ ██╔══██║██║   ██║██║     
    ██║  ██║██║ ╚═╝ ██║██║  ██║███████╗ ╚████╔╝ ██║  ██║╚██████╔╝███████╗
    ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝  ╚═══╝  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝
                        A M A L   V A U L T 
"#;
    println!("{}", ascii);
    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                println!("Entry added successfully!");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    println!("Error reading passwords from file: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service: {}, Username: {}, Password: {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    println!("Error reading passwords from file: {}", err);
                    Vec::new()
                });
                let search = prompt("Search : ");
                for item in &services {
                    if item.service.contains(&search) || item.username.contains(&search) {
                        println!(
                            "Service: {}, Username: {}, Password: {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
