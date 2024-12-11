const INPUT: &str = include_str!(".././input");
use std::collections::HashMap;

fn main() {
    part2();
}

fn part1() {
    let mut stones:Vec<u64> = INPUT.trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    for _i in 0..25 {
        let mut j = 0;
        while j < stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
            }
            else if get_num_digits(&stones[j])%2 == 0 {
                let new_digits = split_digit(&stones[j]);
                stones[j] = new_digits.0;
                stones.insert(j+1, new_digits.1);
                j += 2;
                continue;
            }
            else {
                stones[j] = stones[j]*2024;
            }
            j += 1;
        }
    }
    // println!("{:?}", stones);
    println!("{}", stones.len());
}

fn get_num_digits(x:&u64) -> u64 {
    return x.to_string().len().try_into().unwrap();
}

fn split_digit(x:&u64) -> (u64,u64) {
    let string = x.to_string();
    let string_len:usize = string.len().try_into().unwrap();
    let half_size = (string_len/2).try_into().unwrap();
    let split_string = string.split_at(half_size);
    return (split_string.0.parse::<u64>().unwrap(),
            split_string.1.parse::<u64>().unwrap());
}

fn part2() {
    let mut stones:HashMap<u64,u64> = HashMap::<u64,u64>::new();
    INPUT.trim()
        .split(" ")
        .for_each(|x| {
            let num = x.parse::<u64>().unwrap();
            stones.entry(num).or_default();
            let cur_count:u64 = *stones.get(&num).unwrap();
            stones.insert(num, cur_count+1);
        });
    for _i in 0..75 {
        let mut new_stones:HashMap<u64,u64> = HashMap::<u64,u64>::new();
        for (key, value) in &stones {
            if *key == 0 {
                new_stones.entry(1).or_default();
                let cur_count:u64 = *new_stones.get(&1).unwrap();
                new_stones.insert(1, cur_count+value);
            }
            else if get_num_digits(key)%2 == 0 {
                let new_digits = split_digit(key);
                
                new_stones.entry(new_digits.0).or_default();
                let cur_count:u64 = *new_stones.get(&new_digits.0).unwrap();
                new_stones.insert(new_digits.0, cur_count+value);

                new_stones.entry(new_digits.1).or_default();
                let cur_count:u64 = *new_stones.get(&new_digits.1).unwrap();
                new_stones.insert(new_digits.1, cur_count+value);
            }
            else {
                new_stones.entry(*key*2024).or_default();
                let cur_count:u64 = *new_stones.get(&(key*2024)).unwrap();
                new_stones.insert(*key*2024, cur_count+value);
            }
        }
        stones = new_stones;
    }
    let mut total = 0;
    stones.into_iter().for_each(|(_key,value)| {
        total += value;
    });
    println!("{}", total);
}
