use std::io;

fn main() {
    let mut password = String::new();
    println!("enter a password to test :");
    io::stdin().read_line(&mut password).expect("readline erro");
    let password = password.trim();

    let remarques: [&str; 6] = [
        "impossibly weak",
        "weak",
        "above weak",
        "fifty_fifty",
        "medium",
        "strong",
    ];
    let mut note: usize = 0;

    let mut min: bool = false;
    let mut maj: bool = false;
    let mut digit: bool = false;
    let mut schar: bool = false;

    for c in password.chars() {
        if !schar && c.is_alphanumeric() {
            schar = true;
        } else if !min && c.is_ascii_lowercase() {
            min = true;
        } else if !maj && c.is_ascii_uppercase() {
            maj = true;
        } else if !digit && c.is_ascii_digit() {
            digit = true;
        }
    }

    let len = password.chars().count();
    if len >= 12 {
        note += 2;
    } else if len >= 8 {
        note += 1;
    }
    if digit {
        note += 1;
    }
    if schar {
        note += 1;
    }
    if min && maj {
        note += 1;
    }
    println!(" score : {}/5  {}", note, remarques[note])
}
