enum Light {
    Red,
    Yellow,
    Green,
}

impl Light {
    fn next(&self) -> Light {
        match self {
            Light::Green => Light::Yellow,
            Light::Red => Light::Green,
            Light::Yellow => Light::Red,
        }
    }

    fn name(&self) -> &str {
        match self {
            Light::Green => "Green",
            Light::Red => "Red",
            Light::Yellow => "Yellow",
        }
    }

    fn duration(&self) -> u32 {
        match self {
            Light::Red => 30,
            Light::Yellow => 5,
            Light::Green => 45,
        }
    }
}

fn main() {
    let lights = [Light::Red, Light::Green, Light::Yellow];
    let light1 = lights.get(0).unwrap();
    println!("{} lasts {}s", light1.name(), light1.duration());
    let light2 = light1.next();
    println!("before {}, after {}", light1.name(), light2.name());
    println!("it last : {}", light2.duration());
    let light3 = light2.next();
    println!("third light : {}", light3.name());

    match lights.get(99) {
        Some(l) => println!("light 99, {}", l.name()),
        None => println!("light 99: none"),
    }
}
