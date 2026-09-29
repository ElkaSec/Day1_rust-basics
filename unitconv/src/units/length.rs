const COF: f64 = 0.621371;

pub fn km_to_miles(km: f64) -> f64 {
    km * COF
}
pub fn miles_to_km(m: f64) -> f64 {
    m / COF
}
