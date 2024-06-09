use js_sys::Math;

pub fn random_bool() -> bool {
    Math::random() < 0.5
}

pub fn random_f64(lower_bound: f64, upper_bound: f64) -> f64 {
    (Math::random() * (upper_bound - lower_bound)) + lower_bound
}

pub fn random_i32(lower_bound: i32, upper_bound: i32) -> i32 {
    random_f64(lower_bound as f64, upper_bound as f64) as i32
}

pub fn random_usize(lower_bound: usize, upper_bound: usize) -> usize {
    random_f64(lower_bound as f64, upper_bound as f64) as usize
}

pub fn random_element<T: Clone>(v: Vec<T>) -> Option<T> {
    v.get(random_usize(0, v.len())).cloned()
}