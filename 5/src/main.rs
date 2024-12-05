const INPUT: &str = include_str!(".././input");
use regex::Regex;
use std::collections::HashMap;

fn main() {
    part2();
}

// HashMap of <Number, Numbers to come after>
fn part1() {
    let mut rules: HashMap<i32,Vec<i32>> = HashMap::<i32,Vec<i32>>::new();
    let regex_rules = Regex::new(r"\d+\|\d+").unwrap();

    let mut last_match = 0;
    for capture in regex_rules.captures_iter(INPUT) {
        let m = capture.get(0).unwrap();
        let digits:Vec<&str> = (&INPUT[m.start()..m.end()]).split(r"|").collect();
        let digit1:i32 = digits[0].parse().ok().unwrap();
        let digit2:i32 = digits[1].parse().ok().unwrap();
        if rules.contains_key(&digit1) {
            rules.get_mut(&digit1).expect("").push(digit2);
        } else {
            rules.insert(digit1, vec![digit2]);
        }
    }
    let regex_sequence = Regex::new(r"\n\d+(,\d+)+").unwrap();
    // let mut results:Vec<bool> = vec![];
    let mut result = 0;
    last_match = 0;
    for capture in regex_sequence.captures_iter(INPUT) {
        let m = capture.get(0).unwrap();
        let sequence:Vec<&str> = (&INPUT[m.start()..m.end()].trim()).split(r",").collect();
        let mut cur_sequence:Vec<i32> = vec![];
        let mut is_valid = true;

        // println!("{:?}", sequence);
        for string in &sequence {
            let cur_digit:i32 = string.parse().ok().unwrap();
            // println!("Cur digit {}", cur_digit);
            
            if rules.get(&cur_digit) == None {
                cur_sequence.push(cur_digit);
                continue;
            }
            for rule_num in rules.get(&cur_digit).unwrap() {
                // println!("Rule num {}", rule_num);
                if cur_sequence.contains(rule_num) {
                    println!("Failed {:?} {} must come before {}", &sequence, &cur_digit, &rule_num);
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
            cur_sequence.push(cur_digit);
        }
        if is_valid {
            result += cur_sequence.get(cur_sequence.len()/2).unwrap();
        }
    }
    println!("{}", result);
}

fn part2() {
    let mut rules: HashMap<i32,Vec<i32>> = HashMap::<i32,Vec<i32>>::new();
    let regex_rules = Regex::new(r"\d+\|\d+").unwrap();

    for capture in regex_rules.captures_iter(INPUT) {
        let m = capture.get(0).unwrap();
        let digits:Vec<&str> = (&INPUT[m.start()..m.end()]).split(r"|").collect();
        let digit1:i32 = digits[0].parse().ok().unwrap();
        let digit2:i32 = digits[1].parse().ok().unwrap();
        if rules.contains_key(&digit1) {
            rules.get_mut(&digit1).expect("").push(digit2);
        } else {
            rules.insert(digit1, vec![digit2]);
        }
    }
    let regex_sequence = Regex::new(r"\n\d+(,\d+)+").unwrap();
    let mut failed_sequences:Vec<Vec<i32>> = vec![];
    let mut result = 0;
    for capture in regex_sequence.captures_iter(INPUT) {
        let m = capture.get(0).unwrap();
        let sequence:Vec<i32> = (&INPUT[m.start()..m.end()].trim()).split(r",").map(|x| x.parse::<i32>().unwrap()).collect();
        
        if check_valid(sequence.clone(), rules.clone()).is_some() {
            failed_sequences.push(sequence);
        }
    }
    for sequence in &failed_sequences {
        let mut cur_sequence = sequence.clone();
        let mut is_valid = false;
        while !is_valid {
            let mut build_sequence:Vec<i32> = vec![]; // Can't have string array without borrowing :(
            // println!("Start {:?}", cur_sequence);
            for cur_pos in 0..sequence.len() {
                let mut cur_digit:i32 = cur_sequence.get(cur_pos).unwrap().clone();
                // println!("Cur digit {}", cur_digit);
                
                if rules.get(&cur_digit) == None {
                    build_sequence.push(cur_digit);
                    continue;
                }

                for rule_num in rules.get(&cur_digit).unwrap() {
                    let fail_pos = build_sequence.iter().position(|&i| i == rule_num.clone());
                    if fail_pos.is_some() {
                        // println!("Swap {cur_digit} {rule_num}");
                        build_sequence[fail_pos.unwrap()] = cur_digit.clone();
                        cur_digit = rule_num.clone();
                    }
                }
                build_sequence.push(cur_digit.clone());
            }
            cur_sequence = build_sequence.clone();
            // println!("Stop {:?}", cur_sequence);
            if check_valid(cur_sequence.clone(), rules.clone()).is_none() {
                is_valid = true;
                result += cur_sequence.get(cur_sequence.len()/2).unwrap();
            }
        }
    }
    // println!("{:?}", failed_sequences);
    println!("{}", result);
}
fn check_valid(sequence:Vec<i32>, rules:HashMap<i32,Vec<i32>>) -> Option<usize> {
    let mut cur_sequence:Vec<i32> = vec![];
    // println!("{:?}", sequence);
    for cur_pos in 0..sequence.len() {
        let mut cur_digit:i32 = sequence.get(cur_pos).unwrap().clone();
        // println!("Cur digit {}", cur_digit);
        
        if rules.get(&cur_digit) == None {
            cur_sequence.push(cur_digit);
            // println!("No rule found.");
            continue;
        }

        for rule_num in rules.get(&cur_digit).unwrap() {
            let fail_pos = cur_sequence.iter().position(|&i| i == rule_num.clone());
            if fail_pos.is_some() {
                // println!("Failed {:?} {} must come before {}", &sequence, &cur_digit, &rule_num);
                return fail_pos;
            }
            cur_sequence.push(cur_digit);
        }
    }
    return None;
}