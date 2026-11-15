use std::{io::{self, Write}, thread, time::Duration};

const DELAY: Duration = Duration::from_millis(75);

fn slow_type(text: &str){
    let mut stdout = io::stdout().lock();

    for i in text.chars() {
        write!(stdout, "{i}").unwrap();
        stdout.flush().unwrap();
        thread::sleep(DELAY);
    }
}

enum Characters {
    Candy,
    Jeremy,
    Greg,
    Ted,
    Safra,
    Martha,
    Jackson,
    Flossy,
}



fn main() {
    slow_type("Hello, world!");
}
