use std::collections::HashMap;

fn main() {
    let phrase = ["rust", "is", "fast", "rust", "is", "safe"];
    let mut mHM: HashMap<&str, i32> = HashMap::new();

    for i in phrase {
        let ptr = mHM.entry(i).or_insert(0);
        *ptr += 1;
    }

    let mut vecteur: Vec<_> = mHM.keys().collect();
    vecteur.sort();
    for key in vecteur {
        println!("{key}: {}", mHM[key]);
    }
}
