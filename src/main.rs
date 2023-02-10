use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/*
    Employee database using vectors and hashmaps
*/

fn main() {

    let mut database: HashMap<String, Vec<String>>= HashMap::new();

    // Manually entering the departments with empty employee vector

    database.insert(String::from("Engineering"), Vec::new());
    database.insert(String::from("Sales"), Vec::new());
    database.insert(String::from("Accounting"), Vec::new());
    database.insert(String::from("HR"), Vec::new());
    database.insert(String::from("Legal"), Vec::new());
    
    loop {
        println!("Welcome to Employee Database");
        println!("Please enter a command.");
        println!("Here's an example -- Add Sheldon to Sales");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Cannot read string!");

        let mut count = 0;
        let mut department = String::new();
        let mut employee_name = String::new();
        
        for word in command.split_whitespace() {
            if count == 1 {
                employee_name = word.to_string();
            }
            if count == 3 {
                department = word.to_string();
            }
            count += 1;
        }

        match database.entry(department) {
            Entry::Vacant(e) => { e.insert(vec![employee_name]); },
            Entry::Occupied(mut e) => { e.get_mut().push(employee_name); },
        }

        println!("{:?}", database);
    }
    
}