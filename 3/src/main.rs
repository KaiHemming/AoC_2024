const INPUT: &str = include_str!("../input");
use regex::Regex;

// Part1: mul\(\d{1,3},\d{1,3}\)
// Part2: (mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))

fn main() {
    part2();
}

fn part1() {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let digit_regex = Regex::new(r"\d{1,3}").unwrap();

    let needles: Vec<&str> = regex.find_iter(INPUT).map(|x| x.as_str()).collect();

    let mut results: Vec<i32> = vec![]; 
    for i in 0..needles.len() {
        let digits: Vec<&str> = digit_regex.find_iter(needles[i]).map(|x| x.as_str()).collect();
        let cur_result = digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();
        println!("{} x {} = {}", digits[0], digits[1], cur_result);
        results.push(cur_result);
    }
    let total: i32 = results.iter().sum();
    println!("{}", total);
}

fn part2() {
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let digit_regex = Regex::new(r"\d{1,3}").unwrap();
    let do_regex = Regex::new(r"do\(").unwrap();
    let dont_regex = Regex::new(r"don\'t").unwrap();

    let needles: Vec<&str> = regex.find_iter(INPUT).map(|x| x.as_str()).collect();
    // println!("{:#?}", needles);

    let mut results: Vec<i32> = vec![]; 
    let mut is_multiplying = true;

    for i in 0..needles.len() {
        if do_regex.captures(needles[i]).is_some() {
            println!("do");
            is_multiplying = true;
        }
        else if dont_regex.captures(needles[i]).is_some() {
            println!("don't");
            is_multiplying = false;
        }
        else if is_multiplying {
            let digits: Vec<&str> = digit_regex.find_iter(needles[i]).map(|x| x.as_str()).collect();
            let cur_result = digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();
            println!("{} x {} = {}", digits[0], digits[1], cur_result);
            results.push(cur_result);
        } else {
            println!("Skip");
        }
    }
    let total: i32 = results.iter().sum();
    println!("{}", total);
}