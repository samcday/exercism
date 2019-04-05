#![deny(clippy::all, clippy::pedantic)]

pub fn raindrops(n: u32) -> String {
    let mut iter = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|(factor, label)| if n % factor == 0 { Some(*label) } else { None })
        .peekable();

    match iter.peek() {
        Some(_) => iter.collect::<String>(),
        None => n.to_string(),
    }
}
