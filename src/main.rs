use std::{io::{self, Write, BufReader}, thread, time::Duration};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::error::Error;


const DELAY: Duration = Duration::from_millis(75);
const NUM_CHARACTERS: usize = 8;

//enum to show the possible variants of what character can be chosen, anything different will not be represented
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
enum People {
    Candy,
    Jeremy,
    Greg,
    Ted,
    Safra,
    Martha,
    Jackson,
    Flossy,
}

//Allows for iterating through the list of enums, 'num_characters' constant determines size of 'People::ALL' array
impl People {
    const ALL: [People; NUM_CHARACTERS] = [
        People::Candy,
        People::Jeremy,
        People::Greg,
        People::Ted,
        People::Safra,
        People::Martha,
        People::Jackson,
        People::Flossy,
    ];
}

//Matches each enum to it's appropriate string representation
impl std::fmt::Display for People{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        let s = match self {
            People::Candy => "Candy",
            People::Jeremy => "Jeremy",
            People::Greg => "Greg",
            People::Ted => "Ted",
            People::Safra => "Safra",
            People::Martha => "Martha",
            People::Jackson => "Jackson",
            People::Flossy => "Flossy",
        };
        f.write_str(s)
    }
}

//to represent the structure of each Person json for deserialization
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Person {
    name: People,
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
    slow_type("Hello World!!");

    //return all values
    let data = get_json("Characters.json");

    println!("{data:#?}")

}
