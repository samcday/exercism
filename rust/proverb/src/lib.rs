#![deny(clippy::all, clippy::pedantic)]

pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
        .chain(list.first().map(|want| format!("And all for the want of a {}.", want)))
        .collect::<String>()
}
