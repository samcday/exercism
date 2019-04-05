#![deny(clippy::all, clippy::pedantic)]

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let mut chars = message.chars().rev();
    let last_char = chars.next().unwrap();

    let mut chars = chars.filter(|c| c.is_alphabetic()).peekable();
    let is_shouting = chars.peek().is_some() && chars.all(|c| c.is_uppercase());

    match last_char {
        '?' if is_shouting => "Calm down, I know what I'm doing!",
        '?' => "Sure.",
        _ if is_shouting => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
