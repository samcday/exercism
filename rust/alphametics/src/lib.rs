#![deny(clippy::all, clippy::pedantic)]

use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut operands = input
        .split(|c| c == '=' || c == '+')
        .map(str::trim)
        .filter_map(|operand| {
            if operand.is_empty() {
                return None;
            }
            Some(operand.chars().rev().collect::<Vec<char>>())
        })
        .collect::<Vec<Vec<char>>>();
    if operands.len() < 2 {
        return None;
    }

    let leading_letters = operands
        .iter()
        .map(|op| op.last().unwrap())
        .cloned()
        .collect::<Vec<char>>();

    let uniq_letters = operands
        .iter()
        .flat_map(|v| v.iter())
        .cloned()
        .collect::<HashSet<char>>();
    let mut solution = HashMap::new();
    let sum = operands.pop().unwrap();

    // Need to be more unique letters in operands than in sum.
    if uniq_letters.len() == sum.iter().collect::<HashSet<&char>>().len() {
        return None;
    }

    // Can't be more than 10 unique letters, otherwise we don't have enough unique digits.
    if uniq_letters.len() > 10 {
        return None;
    }

    if sum.len() > operands.iter().map(Vec::len).max().unwrap() {
        // Sum has more digits than longest operand, that means most significant digits has to be a 1.
        // This reduces brute force search space a bit.
        solution.insert(*sum.last().unwrap(), 1);
    }

    let pending_letters = uniq_letters
        .iter()
        .cloned()
        .filter(|letter| !solution.contains_key(letter))
        .collect::<Vec<char>>();

    // Brute force because computer can do numbers fast and stuff. Take all unique letters and put them in a list.
    // Start with the first digit, going from 0-9, pick the first number that isn't already assigned to another letter.
    let mut stack = vec![];
    for letter in &pending_letters {
        let digit = (0..=9).find(|num| !stack.iter().any(|(_, v)| v == num)).unwrap();
        stack.push((*letter, digit));
        solution.insert(*letter, digit);
    }

    while !stack.is_empty() {
        while !stack.is_empty() {
            let (letter, digit) = stack.pop().unwrap();
            solution.remove(&letter);
            let next_digit = (digit + 1..=9).find(|n| !stack.iter().any(|(_, v)| v == n));
            if let Some(next_digit) = next_digit {
                stack.push((letter, next_digit));
                solution.insert(letter, next_digit);
                break;
            }
        }

        if stack.is_empty() {
            // We exhausted the whole search space and came up with nada.
            return None;
        }

        while stack.len() < pending_letters.len() {
            let next_letter = pending_letters[stack.len()];
            let next_digit = (0..=9).find(|n| !stack.iter().any(|(_, v)| v == n)).unwrap();
            solution.insert(next_letter, next_digit);
            stack.push((next_letter, next_digit));
        }

        if check_solution(&solution, &operands, &sum) {
            return Some(solution);
        }
        println!("Combination! {:?}", solution);
    }

    None
}

fn check_solution(letters: &HashMap<char, u8>, operands: &[Vec<char>], sum: &[char]) -> bool {
    u32::from_str_radix(&sum.iter().map(|c| letters[c].to_string()).collect::<String>(), 10).unwrap()
        == operands.iter().fold(0, |acc, oper| {
            acc + u32::from_str_radix(&oper.iter().map(|c| letters[c].to_string()).collect::<String>(), 10).unwrap()
        })
}
