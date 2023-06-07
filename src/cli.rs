use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "morsecode", bin_name = "morsecode", author, version, about, long_about = None)]
/// Morse code translator / cheat sheet
pub struct Cli {
    
    /// Make a subcommand
    #[command(subcommand)]
    pub command: Option<CliCommands>,

    /// File to read from 
    pub file: Vec<std::path::PathBuf>,

    /// File to read to
    #[arg(short = 'o', long = "output")]
    pub output: Option<std::path::PathBuf>,

    /// Display morse code in binary form
    #[arg(short = 'b', long = "binary", global = true)]
    pub binary: bool,
    
    /// Now the command will decode a given Morse Code (both in binary and symbol)
    #[arg(short = 'd', long = "decode", global = true)]
    pub decode: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
pub enum CliCommands {

    /// Display alphabet in morse code
    Alphabet,
}
