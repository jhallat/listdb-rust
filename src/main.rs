use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {

    let properties = load_properties();
    let data_home = properties.get(&String::from("data.home")); 
    match data_home {
        Some(value) => println!("data.home = {}", value),
        None => println!("I have no home")
    }
    loop {
        display_prompt();
        let line = read_line();
        let command_line: Vec<&str> = line.split(' ').collect();
        let command: &str = &command_line[0].to_string().trim().to_uppercase();
        match command {
            "EXIT" => break,
            "CREATE" => create_command(command_line[1..].to_vec()),
            _ => println!("I just don't understand you")
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()    
}

fn display_prompt() {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn create_command(args: Vec<&str>) {

    if args.len() != 2 {
        println!("You messed up!!! Create takes two parameters.")
    }    
    
    for arg in &args {
        println!("{}", arg);
    }
}

fn load_properties() -> HashMap<String, String> {

    let mut properties = HashMap::new();

    let contents = fs::read_to_string("listdb.properties")
        .expect("I can read the properties file :-(");

    //TODO need to read in more than one property
    let property_values: Vec<&str> = contents.split('=').collect();
    properties.insert(property_values[0].to_string(), property_values[1].to_string());

    properties
}

struct Properties {
    property_map: HashMap<String, String>
}

impl Properties {
   fn new() -> Properties {
       Properties {
           property_map: HashMap::new()
       }
   } 

   fn load(&mut self) {
       self.property_map.clear();
       let contents = fs::read_to_string("listdb.properties")
            .expect("I can read the properties file :-(");
       let property_lines: Vec<&str> = contents.split('\n').collect();
        for property_line in property_lines {
            let property_values: Vec<&str> = contents.split('=').collect();
            self.property_map.insert(property_values[0].to_string(), property_values[1].to_string());
        }
   }

   fn get(&self, key: String) -> String {
       match self.property_map.get(&key) {
           Some(value) => value.to_string(),
           None => "".to_string()
       }
       
   }
}