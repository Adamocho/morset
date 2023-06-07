pub const DOT: char = '.';
pub const DASH: char = '-';
pub const DITH: char = DASH;
pub const LETTER_GAP: char = ' ';
pub const WORD_GAP: char = '/';

pub const DOT_BIN: &str = "10";
pub const DASH_BIN: &str = "1110";
pub const DITH_BIN: &str = DASH_BIN;
pub const LETTER_GAP_BIN: &str = "00";
pub const WORD_GAP_BIN: &str = "000000";


pub static MORSE_CODE_SIGNS: [(char, &str); 53]= [
    ('a', ".-"),
    ('b', "-..."),
    ('c', "-.-."),
    ('d', "-.."),
    ('e', "."),
    ('f', "..-."),
    ('g', "--."),
    ('h', "...."),
    ('i', ".."),
    ('j', ".---"),
    ('k', "-.-"),
    ('m', "--"),
    ('n', "-."),
    ('l', ".-.."),
    ('o', "---"),
    ('p', ".--."),
    ('q', "--.-"),
    ('r', ".-."),
    ('s', "..."),
    ('t', "-"),
    ('u', "..-"),
    ('v', "...-"),
    ('w', ".--"),
    ('x', "-..-"),
    ('y', "-.--"),
    ('z', "--.."),
    ('0', "-----"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
    ('&', ".-..."),
    ('\'', ".----."),
    ('@', ".--.-."),
    (')', "-.--.-"),
    ('(', "-.--."),
    (':', "---..."),
    (',', "--..--"),
    ('=', "-...-"),
    ('.', ".-.-.-"),
    ('-', "-....-"),
    ('%', "-----/-...-./-----"),
    ('+', ".-.-."),
    ('\"', ".-..-."),
    ('?', "..--.."),
    ('/', "-..-."),
    (' ', "/"),
    (' ', " "),
];

pub fn display_alphabet(binary: bool) {
    for letter in MORSE_CODE_SIGNS {

        if !binary {
            println!("{:?}\t{}", letter.0, letter.1);
        }
        else {
            let mut new_letter: String = String::new();

            for c in letter.1.chars() {
                match c {
                    DOT => new_letter += DOT_BIN,
                    DASH => new_letter += DASH_BIN,
                    LETTER_GAP => new_letter += LETTER_GAP_BIN,
                    WORD_GAP => new_letter += WORD_GAP_BIN,
                    _ => ()
                }
            }
            println!("{:?}\t{}", letter.0, new_letter);
        }
    }
}

pub fn morse_to_text(message: String, binary: bool) {
        let words: Vec<String> = 
            if binary { message.split("0000000").map(String::from).collect() }
            else { message.split('/').map(String::from).collect() };

        for word in words {
            let letters: Vec<&str> = 
                if binary { word.split("000").collect() }
                else { word.split(' ').collect() };

            for letter in letters {
                let letter_from_bin = bin_to_morse(letter.to_owned());
                for (character, code) in MORSE_CODE_SIGNS {
                    if letter == code || letter_from_bin == code {
                        print!("{}", character.to_uppercase());
                    }
                }     
            }
            print!(" ");
        }
        println!();
}

pub fn text_to_morse(message: String, binary: bool) {
    for c in message.to_lowercase().chars() {
        for (symbol, morse) in MORSE_CODE_SIGNS.into_iter() {
            if c == symbol {
                let mut morse_code: String = morse.to_owned() + " ";
                if binary {
                    morse_code = String::new(); 
                    for ch in morse.chars() {
                        match ch {
                            DOT => morse_code += DOT_BIN,
                            DASH => morse_code += DASH_BIN,
                            WORD_GAP => morse_code += WORD_GAP_BIN,
                            _ => ()
                        }
                    }
                    morse_code += LETTER_GAP_BIN;
                }
                print!("{}", morse_code);
            }
        }
    }
    println!();
}

fn bin_to_morse(bin_letter: String) -> String {
    let mut morse_letter = String::new();
    let mut ones_counter = 0;

    for (index, bit) in bin_letter.chars().enumerate() {
        if bit == '1' {
            ones_counter += 1;
        } 
        if bit == '0' || index == bin_letter.len() - 1 {
            match ones_counter {
                1 => morse_letter += ".",
                3 => morse_letter += "-",
                _ => ()
            }
            ones_counter = 0;
        }
    }
    morse_letter
}