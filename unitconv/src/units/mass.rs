const COF: f64 = 2.20462;

pub fn kg_to_lbs(m: f64) -> f64 {
    m * COF
}

pub fn lbs_to_kg(m: f64) -> f64 {
    m / COF
}
