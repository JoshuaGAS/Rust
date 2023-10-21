#![allow(unused)]

use clap::Parser;
use rand::Rng;

fn main() {
    match do_new_password() {
        Ok(new_pass) => println!("New Password Generated:\n {}", new_pass),
        Err(error) => eprintln!("Error:{}", error),
    }
}

fn do_new_password() -> Result<String, String> {
    let args = Cli::parse();
    let user_input: usize = args.pattern
        .parse()
        .map_err(|e| format!("Invalid input.\n\n PLEASE TYPE JUST NUMBERS!!!!!!!!"))?;
    let mut rng = rand::thread_rng();
    let characters = [
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "g",
        "h",
        "i",
        "j",
        "k",
        "l",
        "m",
        "n",
        "o",
        "p",
        "q",
        "r",
        "s",
        "t",
        "u",
        "v",
        "w",
        "x",
        "y",
        "z",
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
        "T",
        "U",
        "V",
        "W",
        "X",
        "Y",
        "Z",
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "10",
        "~",
        "`",
        "!",
        "@",
        "#",
        "$",
        "%",
        "^",
        "&",
        "*",
        "-",
        "_",
        "+",
        "=",
        ":",
        "|",
        "<",
        ",",
        ">",
        ".",
        "?",
        "/",
    ];
    let mut new_pass = String::new();
    let mut count = 0;

    while count < user_input {
        let random_number = rng.gen_range(0..characters.len());
        new_pass.push_str(characters[random_number]);

        count += 1;
    }
    Ok(new_pass)
}
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
}
