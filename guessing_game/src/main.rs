use rand::Rng;
use std::io;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(1..=100);
    let mut buff_str: String = String::new();

    loop {
        buff_str.clear();
        io::stdin().read_line(&mut buff_str).expect("read number");
        let number: u8 = match buff_str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("try a valid number ");
                continue;
            }
        };
        if number < secret {
            println!("try higher !");
        } else if number > secret {
            println!("try lower !");
        } else {
            println!("bingoooo !");
            break;
        }
    }
}
