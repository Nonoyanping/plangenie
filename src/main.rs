// src/main.rs

use std::io;

mod todo_item;
mod todo_list;

fn main() {
    println!("##################################################");
    println!("############### This is PlanGenie ################");
    println!("##################################################");

    let mut todo_list = todo_list::TodoList::new();

    println!(" ");
    println!("@Enter a number between 1 ~ 5 to do something!@");
    println!("--------------------------------------------------");

    loop {
        println!("1.Add todo.");
        println!("2.Remove todo.");
        println!("3.List todo.");
        println!("4.Toggle todo status.");
        println!("5.Quit");

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
                println!("--------------------------------------------------");
                println!("Enter todos: ");
                let mut text = String::new();
                io::stdin()
                    .read_line(&mut text)
                    .expect("Failed to read line");
                todo_list.add(text.trim().to_string());
                println!("--------------------------------------------------");
            }
            2 => {
                println!("--------------------------------------------------");
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
                println!("--------------------------------------------------");
            }
            3 => {
                println!("--------------------------------------------------");
                println!("Todo List:");
                todo_list.list();
                println!("--------------------------------------------------");
            }
            4 => {
                println!("--------------------------------------------------");
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
                println!("--------------------------------------------------");
            }
            5 => {
                println!("--------------------------------------------------");
                println!("Goodbye! See you next time!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}
