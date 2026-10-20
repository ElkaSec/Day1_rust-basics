use std::env;

enum ParseError {
    ArgsError(usize),
    OutofRange(usize),
    FieldsError(usize),
}
struct IPv4 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}
enum Outcome {
    ParseError(ParseError),
    IPv4(IPv4),
}

impl ParseError {
    fn describe(&self) -> String {
        match self {
            ParseError::ArgsError(n) => format!("ArgsError : u need one argument u showed {}", n),
            ParseError::OutofRange(n) => format!(
                " OutofRangeError :  byte {} is out of range Or not a number ",
                n
            ),
            ParseError::FieldsError(n) => {
                format!(" ParseError : adress needs 4 ranges u only give {}", n)
            }
        }
    }
}

impl IPv4 {
    fn show(&self) -> String {
        format!("IpAddr : {}.{}.{}.{}", self.a, self.b, self.c, self.d)
    }
}

fn parse(mut args: Vec<String>) -> Outcome {
    if args.len() != 2 {
        return Outcome::ParseError(ParseError::ArgsError(args.len()));
    }
    let ip_string = args.remove(1);
    let raw_octes: Vec<&str> = ip_string.split('.').collect();
    if raw_octes.len() != 4 {
        return Outcome::ParseError(ParseError::FieldsError(raw_octes.len()));
    }
    let mut octects = [0u8; 4];

    for i in 0..4 {
        let octet: u8 = match raw_octes[i].parse() {
            Ok(v) => v,
            Err(_) => return Outcome::ParseError(ParseError::OutofRange(i)),
        };
        octects[i] = octet;
    }
    Outcome::IPv4(IPv4 {
        a: (octects[0]),
        b: (octects[1]),
        c: (octects[2]),
        d: (octects[3]),
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = parse(args);
    match result {
        Outcome::IPv4(ip) => println!("{}", ip.show()),
        Outcome::ParseError(e) => println!("{}", e.describe()),
    }
}
