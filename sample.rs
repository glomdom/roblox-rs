fn main(c: i64) -> i64 {
    let x = match c {
        0 => "i love",
        1 | 2 => "women",
        3..=5 => "yeah",
        _ => "smash",
    };
}