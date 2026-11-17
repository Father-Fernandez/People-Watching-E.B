use std::{io::{self, Write, BufReader}, thread, time::Duration};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::error::Error;

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

//finally working!!
fn get_json(name: &str) -> Result<Vec<Person>, Box<dyn Error>>{
    //Load and open json file
    // read it through a buffer so that i can be parsed through a vector of "Person" structs
    let file = File::open(name)?;
    let json = BufReader::new(file);

    let people: Vec<Person> = serde_json::from_reader(json)?;

    //assumed that value passed without error
     Ok(people)
}


fn main() {
    // slow_type("Hello World!!");

    //return all values
    let data = get_json("Characters.json");

    println!("{data:#?}")
}
