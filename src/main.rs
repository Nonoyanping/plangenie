// src/main.rs

use std::io;

mod todo_item;
mod todo_list;

fn main() {
    let mut todo_list = todo_list::TodoList::new();

    loop {
        print!("1. Add todo.");
        print!("2. Remove todo.");
        print!("3. List todo.");
        print!("4. Toggle todo status.");
        print!("5. Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("[Todo]: ");
                let mut text = String::new();
                io::stdin()
                    .read_line(&mut text)
                    .expect("Failed to read line");
                todo_list.add(text.trim().to_string());
            }
            2 => {
                println!("Enter todo ID to remove: ");
                let mut id_str = String::new();
                io::stdin()
                    .read_line(&mut id_str)
                    .expect("Failed to read line");
                let id: usize = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                if let Some(_) = todo_list.remove(id) {
                    println!("Todo removed successfully!");
                } else {
                    println!("Todo with ID {} not found.", id);
                }
            }
            3 => {
                println!("Todo List:");
                todo_list.list();
            }
            4 => {
                println!("Enter todo ID to toggle status: ");
                let mut id_str = String::new();
                io::stdin()
                    .read_line(&mut id_str)
                    .expect("Failed to read line");
                let id: usize = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                todo_list.toggle_status(id);
                println!("Todo status toggled successfully!");
            }
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}
