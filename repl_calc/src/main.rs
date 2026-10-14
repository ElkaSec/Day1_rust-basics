use std::io;
use std::io::Write;

fn main() {
    let mut buffer = String::new();

    println!("REPL calculator: type <a> <op> <b>, exit to quit");
    while buffer.as_str().trim() != "exit" {
        buffer.clear();
        print!("> ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut buffer)
            .expect("usage :<a> <op> <b>");
        let buffer = buffer.trim();
        let mystring: Vec<&str> = buffer.split(" ").collect();
        if mystring.len() != 3 {
            println!("usage :<a> <op> <b>");
            continue;
        }
        let operation = mystring[1];
        let a: i32 = mystring[0].parse().expect("you must give a int");
        let b: i32 = mystring[2].parse().expect("you must give a int");
        let result = match operation {
            "+" => a + b,
            "-" => a - b,
            "/" => {
                if b == 0 {
                    println!("b mus not be zero");
                    continue;
                };
                a / b
            }
            "*" => a * b,
            _ => {
                println!("operation must be /+ - *");
                continue;
            }
        };
        println!("= {result}")
    }
}
