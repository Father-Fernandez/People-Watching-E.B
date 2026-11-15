use std::{io::{self, Write}, thread, time::Duration};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

const DELAY: Duration = Duration::from_millis(75);

//to represent the structure of each Person json for deserialization
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Person {
    name: String,
    age: u8,
    quotes: Vec<String>,
}

//You can read and understand
fn slow_type(text: &str){
    let mut stdout = io::stdout().lock();

    for i in text.chars() {
        write!(stdout, "{i}").unwrap();
        stdout.flush().unwrap();
        thread::sleep(DELAY);
    }
}

fn random_select(){
    todo!();
}

fn get_json(){
    let json = std::fs::read_to_string("Characters.json");
    let characters = serde_json::from_str::<Person>(json);

    if characters.is_ok(){
        slow_type("{:#?}", characters.ok().unwrap());
    }
    else{
        print!("{:#?}", characters.err());
    }
}


fn main() {
    slow_type("Hello World!!");
}
