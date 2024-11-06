use std::env;
use std::fs;
use std::path::Path;
use std::process;

const FORM_FEED: u8 = 12;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("rwc requires at least one file as an argument");
        process::exit(1);
    }

    let file_path: &Path = Path::new(&args[1]);

    if !file_path.exists() {
        eprintln!("{} does not exist", file_path.display());
        process::exit(1);
    }

    if !file_path.is_file() {
        eprintln!("{} is not a file", file_path.display());
        process::exit(1);
    }

    let file_contents: String = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut line_count = 0;
    let mut word_count = 0;
    let mut in_word = false;

    for a_byte in file_contents.bytes() {
        if a_byte == b'\n' {
            line_count += 1;
        }

        if a_byte == b'\t'
            || a_byte == b'\n'
            || a_byte == FORM_FEED
            || a_byte == b'\r'
            || a_byte == b' '
        {
            in_word = false;
        } else if !in_word {
            in_word = true;
            word_count += 1;
        }
    }

    println!("Line count: {}", line_count);
    println!("Word count: {}", word_count);
    println!("Byte count: {}", file_contents.len());
}
