use std::iter::Peekable;
use std::str::Chars;

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut rle_state = (None, 0);

    for char in source.chars() {
        match rle_state {
            (None, _) => rle_state = (Some(char), 1),
            (Some(rle_char), ref mut count) if char == rle_char => *count += 1,
            (Some(prev_char), prev_count) => {
                encoded += &format!(
                    "{}{}",
                    if prev_count > 1 {
                        prev_count.to_string()
                    } else {
                        String::new()
                    },
                    prev_char
                );
                rle_state = (Some(char), 1);
            }
        }
    }

    if let (Some(char), count) = rle_state {
        encoded += &format!("{}{}", if count > 1 { count.to_string() } else { String::new() }, char);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut iter = source.chars().peekable();

    // If the stdlib had a peek-friendly take_while, this would be a lot simpler...
    let parse_num = |iter: &mut Peekable<Chars>| {
        let mut num_str = String::new();
        while let Some(char) = iter.peek() {
            if !char.is_ascii_digit() {
                break;
            }
            num_str.push(iter.next().unwrap());
        }
        num_str.parse::<u32>().unwrap_or(0)
    };

    while let Some(_) = iter.peek() {
        let num = std::cmp::max(parse_num(&mut iter), 1);
        let char = iter.next().unwrap();
        decoded += &char.to_string().repeat(num as usize);
    }
    decoded
}
