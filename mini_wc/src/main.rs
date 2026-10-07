use std::collections::HashMap;

fn main() {
    let phrase = ["rust", "is", "fast", "rust", "is", "safe"];
    let mut mhm: HashMap<&str, i32> = HashMap::new();

    for i in phrase {
        let ptr = mhm.entry(i).or_insert(0);
        *ptr += 1;
    }

    let mut vecteur: Vec<_> = mhm.keys().collect();
    vecteur.sort();
    for key in vecteur {
        println!("{key}: {}", mhm[key]);
    }
}
