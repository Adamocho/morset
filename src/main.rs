use std::{path::Path, fs::File, io::{self, BufRead}};

use morset::morse::{self, morse_to_text, text_to_morse};
use morset::cli::{Cli, CliCommands};
use clap::Parser;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

fn main() {
    let arguments = Cli::parse(); 

    if let Option::Some(com) = arguments.command {
        match com {
            CliCommands::Alphabet => morse::display_alphabet(arguments.binary),
        }
        return
    }

    // If there is no command given, get going with text/file input
    if !arguments.file.is_empty() {
        for f in arguments.file {
            if let Ok(lines) = read_lines(&f) {
                println!("file: {:?}", f);

                for line in lines.flatten() {
                    if arguments.decode {
                        morse_to_text(line);
                    } else {
                        text_to_morse(line, arguments.binary); 
                    }
                }
            }
        }
    } else {
        let stdin = std::io::stdin()
            .lock()
            .lines();

        for line in stdin.flatten() {
            if arguments.decode {
                morse_to_text(line);
            } else {
                text_to_morse(line, arguments.binary); 
            }
        }
    }
}
