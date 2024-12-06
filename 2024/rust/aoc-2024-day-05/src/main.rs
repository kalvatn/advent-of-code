use lib::read_lines_from_file;
use lib::read_lines_from_string;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

const TEST_INPUT: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
---
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let input = if args.len() < 2 {
        read_lines_from_string(TEST_INPUT)
    } else {
        read_lines_from_file(&args[1]).unwrap()
    }.join("\n");

    let parts: Vec<&str> = input.split("---\n").collect();
    let ordering: Vec<&str> = parts[0].lines().collect();
    let updates: Vec<Vec<u32>> = parts[1].lines().map(|line| {
        line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect();

    let mut must_be_printed_before: HashMap<u32, Vec<u32>> = HashMap::new();
    ordering.iter().for_each(|line| {
        let parts: Vec<&str> = line.split("|").collect();
        let first = parts[0].parse::<u32>().unwrap();
        let second = parts[1].parse::<u32>().unwrap();
        must_be_printed_before.entry(first).or_insert(Vec::new()).push(second);
    });

    let correct = updates
        .iter()
        .filter(|update| {
            is_correctly_ordered(update, &must_be_printed_before)
        }).cloned().collect();

    let sum = sum_middle_elements(correct);

    println!("Part 1: {}", sum);

    let corrected = updates
        .iter()
        .filter(|update|
            !is_correctly_ordered(update, &must_be_printed_before))
        .map(|update|
            fix_ordering(&update, &must_be_printed_before))
        .collect();

    let sum = sum_middle_elements(corrected);

    println!("Part 2: {}", sum);
    println!("Time: {}ms ({}µs)", timer.elapsed().as_millis(), timer.elapsed().as_micros());
}

fn sum_middle_elements(updates: Vec<Vec<u32>>) -> usize {
    let sum: usize = updates.iter().map(|update| {
        let middle_index = update.len() / 2;
        let middle_element = update[middle_index];
        middle_element as usize
    }).sum();
    sum
}

fn is_correctly_ordered(update: &Vec<u32>, must_be_printed_before: &HashMap<u32, Vec<u32>>) -> bool {
    for i in 0..update.len() {
        let page = update[i];
        if let Some(must_be_after) = must_be_printed_before.get(&page) {
            if update[0..i].iter().any(|&prev_page| must_be_after.contains(&prev_page)) {
                return false;
            }
        }
    }
    true
}

fn fix_ordering(update: &Vec<u32>, must_be_printed_before: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    if is_correctly_ordered(&update, must_be_printed_before) {
        return update.clone();
    }
    let mut result = update.iter().cloned().collect::<Vec<u32>>();
    for i in 1..result.len() {
        let prev = result[i - 1];
        let current = result[i];
        if let Some(must_be_after) = must_be_printed_before.get(&current) {
            if must_be_after.contains(&prev) {
                result.swap(i - 1, i);
            }
        }
    }
    fix_ordering(&result, must_be_printed_before)
}