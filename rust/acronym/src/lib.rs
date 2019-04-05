#![deny(clippy::all, clippy::pedantic)]

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == ':')
        .filter_map(|word| {
            if word.is_empty() {
                return None;
            }
            if word.chars().all(char::is_uppercase) {
                return Some(word[0..1].to_string());
            }
            let mut chars = word.chars();
            let first_char = chars.next().map(|c| c.to_ascii_uppercase());
            Some(
                first_char
                    .iter()
                    .cloned()
                    .chain(chars.filter(|c| c.is_uppercase()).by_ref())
                    .collect::<String>(),
            )
        })
        .collect::<String>()
}
