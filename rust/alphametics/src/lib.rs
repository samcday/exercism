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

    // Leading letter of each operand can't be 0. Reduces brute-force search space a bit.
    let cant_be_zero = operands
        .iter()
        .map(|op| op.last().unwrap())
        .cloned()
        .collect::<HashSet<char>>();

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
        // Sum has more digits than longest operand, that means most significant digit has to be a 1
        // This reduces brute force search space a bit.
        solution.insert(*sum.last().unwrap(), 1);
    }

    // Now we just brute-force the search space, because I suck at math and I'm lazy.

    // depth-first recursive search.
    fn solver<F>(pending_solution: &mut HashMap<char, u8>, letters: &HashSet<char>, operands: &Vec<Vec<char>>, sum: &Vec<char>, possible_digits: &F) -> Option<HashMap<char, u8>>
        where F: Fn(char, &HashMap<char, u8>) -> Vec<u8> {
        // Find the first unsolved letter that hasn't got a value plugged in.
        let next = letters.iter().find(|x| !pending_solution.contains_key(x));
        if next.is_none() {
            // Full pending solution. Maybe this is the one?!
            if check_solution(pending_solution, &operands, &sum) {
                return Some(pending_solution.clone());
            }
            return None;
        }
        let letter = next.unwrap();
        for num in possible_digits(*letter, pending_solution) {
            pending_solution.insert(*letter, num);
            if let Some(solution) = solver(pending_solution, letters, operands, sum, possible_digits) {
                return Some(solution);
            }
        }
        None
    };

    solver(&mut solution, &uniq_letters, &operands, &sum, &|letter: char, solution: &HashMap<char, u8>| {
        (0u8..=9).filter(|n| !(*n == 0 && cant_be_zero.contains(&letter)) && !solution.values().any(|v| v == n)).collect()
    })
}

fn check_solution(letters: &HashMap<char, u8>, operands: &[Vec<char>], sum: &[char]) -> bool {
    u32::from_str_radix(
        &sum.iter().rev().map(|c| letters[c].to_string()).collect::<String>(),
        10,
    )
    .unwrap()
        == operands.iter().fold(0, |acc, oper| {
            acc + u32::from_str_radix(
                &oper.iter().rev().map(|c| letters[c].to_string()).collect::<String>(),
                10,
            )
            .unwrap()
        })
}
