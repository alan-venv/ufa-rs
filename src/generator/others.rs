use rand::Rng;
use uuid::Uuid;

pub fn uuid() -> String {
    return Uuid::new_v4().to_string();
}

pub fn number(min: i64, max: i64) -> String {
    let mut rng = rand::rng();
    let number = rng.random_range(min..max + 1);
    return format!("{}", number);
}

pub fn digits(size: i128) -> String {
    let mut rng = rand::rng();
    let mut min: i128 = 1;
    let mut max: i128 = 10;
    let mut i = 1;
    while i < size && i < 38 {
        min = min * 10;
        max = max * 10;
        i = i + 1;
    }
    let number = rng.random_range(min..max);
    return format!("{}", number);
}
