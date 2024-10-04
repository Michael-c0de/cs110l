// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::seq::index;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::iter::FromIterator;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn find(v: &mut Vec<char>, target: char) -> usize {
    let mut index = 0;
    while index < v.len() {
        if target == v[index] {
            break;
        }
        index += 1;
    }
    return index;
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut target: Vec<char> = Vec::from(['_'].repeat(secret_word.len()));

    let mut guess = String::new();
    loop {
        io::stdout().flush().expect("Error flushing stdout");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let guess_chars: Vec<char> = guess.chars().collect();
        let guess_char = guess_chars[0];
        if secret_word_chars.contains(&guess_char) {
            let index: usize = find(&mut secret_word_chars, guess_char);
            secret_word_chars[index] = '_';
            target[index] = guess_char;
        }
        println!("the world so far is {}", String::from_iter(target.iter()));
        guess.clear();

        if !target.contains(&'_') {
            break;
        }
    }
    print!(
        "Congratulations you guessed the secret word: {}!",
        secret_word
    );
}
