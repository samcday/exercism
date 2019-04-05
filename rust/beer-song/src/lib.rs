#![deny(clippy::all, clippy::pedantic)]

pub fn verse(n: u32) -> String {
    let how_many_bottles = |num: u32| match num {
        0 => "No more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", num),
    };

    format!(
        "{bottles} of beer on the wall, {bottles_lower} of beer.\n{next_steps}, {remaining} of beer on the wall.\n",
        bottles = how_many_bottles(n),
        bottles_lower = how_many_bottles(n).to_lowercase(),
        next_steps = match n {
            0 => "Go to the store and buy some more",
            1 => "Take it down and pass it around",
            _ => "Take one down and pass it around",
        },
        remaining = match n {
            0 => how_many_bottles(99),
            _ => how_many_bottles(n - 1),
        }.to_lowercase()
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .enumerate()
        .map(|(idx, n)| String::from(if idx == 0 { "" } else { "\n" }) + &verse(n))
        .collect::<String>()
}
