use ferris_says::say; // Importing the function Say from Ferris Says
use std::io::{stdout, BufWriter};

fn main(){
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}