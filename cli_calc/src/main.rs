use std::env;
use std::process::exit;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("arguments error ");
        exit(2);
    }

    let a: i32 = args[1].parse().expect("u must give a int ");
    let sign = &args[2];
    let b: i32 = args[3].parse().expect("u must give a int");

    let rsult = match sign.as_str() {
        "+" => a + b,
        "-" => a - b,
        "/" => {
            if b == 0 {
                println!("error deviding by zero");
                exit(1)
            }
            a / b
        }
        "*" => a * b,
        __ => {
            println!("error sign");
            exit(1)
        }
    };
    println!("{}", rsult);
}
