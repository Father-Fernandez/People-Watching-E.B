use std::{io::{self, Write, BufReader}, thread, time::Duration};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::error::Error;
use rand::seq::IndexedRandom;

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
    const ALL: [Self; NUM_CHARACTERS] = [
        Self::Candy,
        Self::Jeremy,
        Self::Greg,
        Self::Ted,
        Self::Safra,
        Self::Martha,
        Self::Jackson,
        Self::Flossy,
    ];
}

//Matches each enum to it's appropriate string representation
impl std::fmt::Display for People{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        let s = match self {
            Self::Candy => "Candy",
            Self::Jeremy => "Jeremy",
            Self::Greg => "Greg",
            Self::Ted => "Ted",
            Self::Safra => "Safra",
            Self::Martha => "Martha",
            Self::Jackson => "Jackson",
            Self::Flossy => "Flossy",
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

fn random_person(people: &[Person]) -> Option<&Person> {
    people.choose(&mut rand::rng())
}

fn random_quote(people: &Person) -> Option<&str> {
    people.quotes.choose(&mut rand::rng()).map(String::as_str)
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

fdsafdsaf
fn main() -> Result<(), Box<dyn Error>> {
    let people = get_json("Characters.json")?;

    let person = random_person(&people).ok_or("no characters loaded")?;
    let quote = random_quote(person).ok_or("sgfdsgfdsgfdsg character has no quotes")?;
gfdsgfd
    println!("\nHey i'm {} been here for about {} years and i just wanna tell you", person.name, person.age);
    println!("\n");dasgfdsagfd
    slow_type(quote);

    Ok(())
}
