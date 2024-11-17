use std::env;
use std::fs;
use std::path::Path;

const FORM_FEED: u8 = 12;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(String::from(
            "rwc requires at least one file as an argument",
        ));
    }

    let file_path: &Path = Path::new(&args[1]);

    if !file_path.exists() {
        return Err(format!("{} does not exist", file_path.display()));
    }

    if !file_path.is_file() {
        return Err(format!("{} is not a file", file_path.display()));
    }

    let file_contents: String = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(format!("{}", err)),
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
    Ok(())
}
