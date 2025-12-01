use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Contact {
    name: String,
    mobno: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage:
  cargo run -- add <name> <mobno>
  cargo run -- delete <name> <mobno>
  cargo run -- search <name>
  cargo run -- search <mobno>
  cargo run -- list"
        );
        return;
    }

    let command = &args[1];

    let mut contacts: Vec<Contact> = load_contacts();

    // add command
    if command == "add" {
        let name = args[2].clone();
        let mobno = args[3].clone();

        let contact = Contact { name, mobno };

        contacts.push(contact);

        for c in &contacts {
            println!("{} : {}", c.name, c.mobno);
        }

        save_contacts(&contacts);

        // delete command
    } else if command == "delete" {
        let name_to_delete = &args[2];
        let mobno_to_delete = &args[3];

        let initial_len = contacts.len();

        contacts.retain(|c| !(c.name == *name_to_delete && c.mobno == *mobno_to_delete));
        if contacts.len() < initial_len {
            println!("Deleted contact: {} : {}", name_to_delete, mobno_to_delete);
        } else {
            println!(
                "Contact not found: {} : {}",
                name_to_delete, mobno_to_delete
            );
        }
        save_contacts(&contacts);

        //search command
    } else if command == "search" {
        let mut found = false;

        for c in &contacts {
            if c.name == args[2] || c.mobno == args[2] {
                println!("Available contact -> {} : {}", c.name, c.mobno);
                found = true
            }
        }
        if !found {
            println!("No contact found matching : {}", args[2]);
        }

        //list command
    } else if command == "list" {
        if contacts.is_empty() {
            println!("No contact in your contact list")
        }

        println!("Your contact list :");

        for c in &contacts {
            println!("{} : {}", c.name, c.mobno);
        }
    } else {
        eprintln!("Unknown command: {}", command);
    }
}

fn save_contacts(contacts: &Vec<Contact>) {
    let json = serde_json::to_string(&contacts).unwrap();
    fs::write("contacts.json", json).unwrap();
}

fn load_contacts() -> Vec<Contact> {
    if let Ok(data) = fs::read_to_string("contacts.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
