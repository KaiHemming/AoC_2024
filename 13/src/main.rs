const INPUT: &str = include_str!(".././input");
use std::collections::HashSet;
use regex::Regex;

//  161619590963441 too high
fn main() {
    part2();
}

fn part1() {
    let separator = Regex::new(r"\n[\s*$]").expect("");
    let button_digits = Regex::new(r"[X|Y]\+(\d+)").expect("");
    let prize_digits = Regex::new(r"[X|Y]=(\d+)").expect("");
    let mut entries:Vec<Vec<i64>> = vec![];
    
    separator.split(INPUT).into_iter().for_each(|entry| {
        entry.trim().lines().for_each(|line| {
            let mut captures: Vec<i64> = vec![];
            for (_, [digit]) in button_digits.captures_iter(line).map(|c| c.extract()) {
                captures.push(digit.parse::<i64>().unwrap());
            }
            for (_, [digit]) in prize_digits.captures_iter(line).map(|c| c.extract()) {
                captures.push(digit.parse::<i64>().unwrap());
            }
            entries.push(captures);
        })
    });

    // println!("{:?}", entries);
    let mut score = 0;
    for i in (0..entries.len()).step_by(3) {
        let gcd_x:i64 = gcd_of_three(entries[i][0], entries[i+1][0], entries[i+2][0]);
        let gcd_y:i64 = gcd_of_three(entries[i][1], entries[i+1][1], entries[i+2][1]);
        let button_a:(i64,i64) = (entries[i][0] / gcd_x, entries[i][1] / gcd_y);
        let button_b:(i64,i64) = (entries[i+1][0] / gcd_x, entries[i+1][1] / gcd_y);
        let prize:(i64,i64) = (entries[i+2][0] / gcd_x, entries[i+2][1] / gcd_y);
        // println!("Problem (after gcd) \n{:?}\n{:?}\n{:?}", button_a, button_b, prize);

        let mut found = false;
        let mut answer:(i64,i64) = (0,0);
        for a_presses in 0..999 {
            for b_presses in 0..999 {
                let total = (
                    button_a.0 * a_presses + button_b.0 * b_presses,
                    button_a.1 * a_presses + button_b.1 * b_presses);
                if (total.0 > prize.0) |
                    (total.1 > prize.1) {
                        break;
                    }
                if total == prize {
                    found = true;
                    answer = (a_presses, b_presses);
                    break;
                }
            }
            let total = (
                button_a.0 * a_presses,
                button_a.1 * a_presses);
            if (total.0 > prize.0) |
                (total.1 > prize.1) {
                    break;
                }
            if found {break;}
        }
        // if found {
        //     println!("Answer: {:?}", answer);
        // }
        score += (answer.0*3) + answer.1;
    }
    println!("{}", score);
}

fn part2() {
    let separator = Regex::new(r"\n[\s*$]").expect("");
    let button_digits = Regex::new(r"[X|Y]\+(\d+)").expect("");
    let prize_digits = Regex::new(r"[X|Y]=(\d+)").expect("");
    let mut entries:Vec<Vec<i64>> = vec![];
    
    separator.split(INPUT).into_iter().for_each(|entry| {
        entry.trim().lines().for_each(|line| {
            let mut captures: Vec<i64> = vec![];
            for (_, [digit]) in button_digits.captures_iter(line).map(|c| c.extract()) {
                captures.push(digit.parse::<i64>().unwrap());
            }
            for (_, [digit]) in prize_digits.captures_iter(line).map(|c| c.extract()) {
                captures.push(digit.parse::<i64>().unwrap());
            }
            entries.push(captures);
        })
    });

    let mut score = 0;
    for i in (0..entries.len()).step_by(3) {
        let button_a:(i64,i64) = (entries[i][0], entries[i][1]);
        let button_b:(i64,i64) = (entries[i+1][0], entries[i+1][1]);
        let prize:(i64,i64) = ((entries[i+2][0]+10000000000000), 
                                (entries[i+2][1]+10000000000000));
        // [94, 34]
        // [22, 67]
        // [8400, 5400]
        let a_presses = ((prize.1*button_b.0)-(prize.0*button_b.1))/
                        ((button_b.0*button_a.1)-(button_a.0*button_b.1));
        let b_presses = (prize.1-(a_presses*button_a.1))/button_b.1;
        // println!("Problem \n{:?}\n{:?}\n{:?}", button_a, button_b, prize);
        let check = ((button_a.0*a_presses) + (button_b.0*b_presses), (button_a.1*a_presses) + (button_b.1*b_presses));
        if prize == check {
            println!("\nA:{}\nB:{}\n", a_presses, b_presses);
            score += a_presses*3 + b_presses;
        }
        else {
            println!("\nNo answer found.\n");
        }
    }
    println!("{}", score);
}

// these were pointless 
fn mult_tuple(multiplier: i64, tuple: (i64,i64)) -> (i64,i64) {
    return (tuple.0*multiplier,tuple.1*multiplier);
}
fn gcd_of_three(x: i64, y: i64, z:i64) -> i64 {
    let gcd_1 = gcd(x,y);
    return gcd(gcd_1, z);
}
// Euclidean algorithm
fn gcd(mut x:i64, mut y:i64) -> i64 {
    // Force y to be bigger than
    if y < x {
        let temp = x;
        x = y;
        y = temp;
    }
    let mut pair = (x, y);
    while pair.0 > 0 {
        let calc = pair.1 % pair.0;
        pair = (calc, pair.0);
    }
    return pair.1;
}