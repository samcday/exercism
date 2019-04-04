pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c), // New opening bracket
            ']' | '}' | ')' => {
                // Closing bracket. Ensure it matches expected.
                let expected = match stack.pop() {
                    Some('(') => ')',
                    Some('[') => ']',
                    Some('{') => '}',
                    _ => return false,
                };
                if expected != c {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty() // We're not balanced if there's remaining open brackets.
}
