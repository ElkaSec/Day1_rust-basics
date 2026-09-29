mod units;

use std::env;
use std::process::exit;
use units::length::km_to_miles;
use units::length::miles_to_km;
use units::mass::kg_to_lbs;
use units::mass::lbs_to_kg;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 3 {
        exit(2);
    }

    let symbole = &args[1];
    let valeur = match args[2].parse::<f64>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("erreur de parsing {}", e);
            exit(3)
        }
    };
    match symbole.as_str() {
        "km" => println!("{} km  in miles   : {}", valeur, km_to_miles(valeur)),
        "miles" => println!("{} miles  in km   : {}", valeur, miles_to_km(valeur)),
        "kg" => println!(" {} kg en lbs :{}", valeur, kg_to_lbs(valeur)),
        "lb" => println!(" {} lb en kg : {}", valeur, lbs_to_kg(valeur)),
        _ => println!(" try only :{} km/miles or lb/kg valeur", args[0]),
    }
}
