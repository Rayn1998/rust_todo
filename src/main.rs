use serde::{ Deserialize, Serialize };
use serde_json;
use std::fs::File;
use std::io::{stdin, Error, Read, Write };
use std::process;

/*
Program shows all the items with marks if they're done or not
Type "add ..." to make program add some item to the list
Type "remove .../all" to delete the item(s)
Type "done ..." to change item's status to "done"
Type "save" to rewrite the "todo.json"
If the "save" was not made it must ask if the user wants to save the changes or leave the old todo list
Type "exit" to exit the program
*/

fn main() {
    todo();
}

// TODO
fn todo() {
    let mut been_saved: State = State::new();

    // Importing the file or creating the new one if nothing exists
    let file: Result<File, Error> = 'todo: loop {
        match File::open("todo.json") {
            Ok(data) => {
                break 'todo Ok(data)
            },
            Err(_) => {
                println!("There were no \"todo\", so the new one will be created");
                create_todo_json();
                continue 'todo;
            }
        };
    };

    let mut file = match file {
        Ok(data) => {
            data
        },
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    };

    // Printing the items from file
    let mut starter_items = String::new();
    file.read_to_string(&mut starter_items).unwrap();
    let mut items: Vec<Item> = match serde_json::from_str(&starter_items) {
        Ok(items) => items,
        Err(_) => {
            println!("File is empty or contains invalid data. Starting with an empty list.");
            Vec::new()
        }
    };
    print_items(&mut items);

    loop {

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read the command");

        // ADD COMMAND
        if input.contains("add") {

            been_saved.set_false();
            let new_item_name: String = input.replace("add ", "").trim().to_string();
            let new_item = Item {
                item: new_item_name,
                status: false,
            };
            items.push(new_item);
            print_items(&mut items);

        // REMOVE COMMAND
        } else if input.contains("remove") {

            let item_id: String = input.replace("remove ", "").trim().to_string();

            if item_id == "all" {
                items.clear();
                println!("All items were removed");
            } else {
                let item_id = match item_id.parse::<i32>() {
                    Ok(data) => data,
                    Err(_) => {
                        println!("Type something");
                        continue;
                    }
                };
                
                println!("{}", item_id);
                if item_id > 0 && item_id <= items.len() as i32 {
                    items.remove((item_id - 1) as usize);
                    been_saved.set_false();
                    print_items(&mut items);
                } else {
                    println!("Enter the correct number for remove");
                    continue;
                }
            }
            
        // DONE COMMAND
        } else if input.contains("done") {

            been_saved.set_false();
            let item_id: u32 = input.replace("done ", "").trim().parse().unwrap();
            let item = items.get_mut((item_id - 1) as usize).unwrap();
            if item.status == false {
                item.status = true;
            } else {
                println!("It's already done) You can delete it, if you want");
            }
            print_items(&mut items);

        // SAVE COMMAND
        } else if input.trim() == "save" {

            if been_saved.get_value() {
                println!("You've already saved it");
            } else {
                save_changes_to_json(&items); 
                been_saved.set_true();           
            }

        // EXIT COMMAND
        } else if input.contains("exit") {

            if been_saved.get_value() {
                State::exit();
            } else {
                println!("You didn't save the changes. Do you want to save it now? y/n");
                'exit_save: loop {
                    let mut answer: String = String::new();
                    stdin().read_line(&mut answer).expect("Error reading the answer");
                    match answer.trim() {
                        "y" => {
                            save_changes_to_json(&items);
                            State::exit();
                        },
                        "n" => {
                            State::exit();
                        },
                        _ => {
                            println!("Type \"y\" or \"n\", please");
                            continue 'exit_save;
                        }
                    }
                }
            }
            
        // HELP COMMAND
        } else if input.contains("help") {

            println!("- add\n- remove\n- done\n- save\n- exit");

        // ANYTHING ELSE
        } else {

            println!("There's no such command, please enter again");
            continue;
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    item: String,
    status: bool,
}

struct State {
    value: bool,
}

impl State {
    fn new() -> State {
        State {
            value: true
        }
    }
    fn set_false(&mut self) -> () {
        self.value = false;
    }
    fn set_true(&mut self) -> () {
        self.value = true;
    }
    fn get_value(&self) -> bool {
        self.value
    }
    fn exit() -> () {
        process::exit(1);
    }
}


fn print_items(items: &mut Vec<Item>) -> () {
    if items.len() > 0 {
        for (index, item) in items.iter_mut().enumerate() {
            if item.status {
                println!("{} - {} +++ done",index + 1, item.item);
            } else {
                println!("{} - {}",index + 1, item.item);
            }
        }
    }
}

fn create_todo_json() -> () {
    match File::create("todo.json") {
        Ok(_) => {
            println!("The new \"todo\" file was created");
        },
        Err(err) => {
            println!("Unable to create a file: {}", err);
        }
    }
}

fn save_changes_to_json(items: &Vec<Item>) {
    let serialized: String = serde_json::to_string(&items).expect("Failed to serialize");
    let serialized = serialized.as_bytes();
    let mut file = File::create("todo.json").unwrap();
    file.write(&serialized).unwrap();
    println!("All has been successfully saved)");
}