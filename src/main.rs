use rand::seq::IteratorRandom;
use std::io;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

//Location path of couple word
const DICT_PATH: &str = "src/dict.txt";

fn main() {
    loop {
        // Bring couples 
        let (first_word, second_word) = detached_words(rand_dictword());
        // Decided which word is first 
        let current_word = first_word;
        println!("{}", current_word);
        // Wait input word from user
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // Compare two words and print result
        match guess.trim() == current_word {
            true => {
                println!("\nYou are cool!\nNext word:\n");
            }, 
            _ => println!("\nWrong try it again\n")
        }
    }
}
// Find random line of words in .txt file.
fn rand_dictword() -> String {
    let f = File::open(DICT_PATH)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", DICT_PATH, e));
    let f = BufReader::new(f);
    let lines = f.lines()
        .map(|l| l.expect("Couldn't read line"));
    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}
// Separate one word from another
fn detached_words(s: String) -> (String, String) {
    let f_word = s.split_whitespace().next().unwrap_or("");
    let s_word = s.replace(&f_word,"");
    (f_word.to_string(), s_word.trim().to_string())
}
