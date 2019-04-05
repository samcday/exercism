#![deny(clippy::all, clippy::pedantic)]

pub fn abbreviate(phrase: &str) -> String {
    if let Some(word) = phrase.split(':').next() {
        if word.chars().all(|c| c.is_uppercase()) {
            return word.to_string();
        }
    }

    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .map(|word| {
            if word.chars().all(char::is_uppercase) {
                return word[0..1].to_string();
            }
            let mut chars = word.chars();
            let first_char = chars.next().map(|c| c.to_ascii_uppercase());
            first_char
                .iter()
                .cloned()
                .chain(chars.filter(|c| c.is_uppercase()).by_ref())
                .collect::<String>()
        })
        .collect::<String>()
}
