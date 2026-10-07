use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_response = "encrypt/decrypt 'String' nb_wrap ";

    if args.len() != 4 {
        println!("{} {}", args[0], error_response);
        exit(2);
    }

    let shift: i64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("{} {}", args[0], error_response);
            exit(2);
        }
    };

    if args[1] == "encrypt" {
        let mut str_sortie = String::new();
        let chars: Vec<char> = args[2].chars().collect();
        for ch_r in chars {
            str_sortie.push(shift_char(ch_r, shift));
        }
        println!("{}", str_sortie.to_string());
        exit(0);
    }

    if args[1] == "decrypt" {
        let mut str_sortie = String::new();
        let chars: Vec<char> = args[2].chars().collect();
        for ch_r in chars {
            str_sortie.push(shift_char(ch_r, -shift));
        }
        println!("{}", str_sortie.to_string());
        exit(0);
    }

    println!("{} {}", args[0], error_response);
    exit(2);
}

fn shift_char(c: char, shift: i64) -> char {
    if c.is_ascii_lowercase() {
        let position = (c as u8 - 'a' as u8) as i64;
        let nouvelle = (position + shift).rem_euclid(26) as u8;
        return (nouvelle + 'a' as u8) as char;
    } else if c.is_ascii_uppercase() {
        let position: i64 = (c as u8 - 'A' as u8) as i64; // hna 0 ->25
        let nouvelle: u8 = (position + shift).rem_euclid(26) as u8;
        return (nouvelle + 'A' as u8) as char;
    } else {
        return c;
    }
}
