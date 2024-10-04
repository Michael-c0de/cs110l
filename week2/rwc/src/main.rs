use regex::Regex;
use std::env;
use std::fs::{File}; // For read_file_lines()
use std::io::{self, BufRead};
use std::process; // For read_file_lines()

/// Reads the file at the supplied path, and returns a vector of strings.
// #[allow(unused)] // TODO: delete this line when you implement this function
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
    let f = File::open(filename)?;
    let mut lines = Vec::new();
    let reader = io::BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(content) => lines.push(content),
            Err(e) => return Err(e),
        }
    }
    Ok(lines)
}

fn read_file(filename: &String) -> Vec<String> {
    match read_file_lines(filename) {
        Ok(lines) => lines,
        Err(e) => {
            println!("Failed to open{}", e);
            Vec::new()
        }
    }
}

fn main() {
    let re = Regex::new(r"\s+").unwrap(); // 匹配空白字符

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename: &String = &args[1];
    let lines = read_file(filename);
    //count of words lines and chars
    let mut c_words = 0;
    let mut c_lines = 0;
    let mut c_chars = 0;
    for line in &lines {
        c_lines += 1;
        for word in re.split(&line) {
            c_words += 1;
            c_chars += word.len();
        }
    }

    println!(
        "filename {}, lines {}, words {}, chars {}",
        filename, c_lines, c_words, c_chars
    );
}
