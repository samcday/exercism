pub fn raindrops(n: u32) -> String {
    let mut iter = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|(factor, label)| match n % factor == 0 {
            true => Some(*label),
            false => None,
        })
        .peekable();

    match iter.peek() {
        Some(_) => iter.collect::<String>(),
        None => n.to_string(),
    }
}
