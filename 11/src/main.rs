const INPUT: &str = include_str!(".././input");
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;

fn main() {
    for i in 0..255 {
        let start = Instant::now();
        // let stones = part2(i);
        let stones = part3(i, 20);
        let elapsed = start.elapsed();

        println!("{}, {:?}, {}", i, elapsed, stones);
    }
    // find_best_def_expensive();
}

fn find_best_def_expensive() {
    let mut cur_time: Duration = Duration::MAX;
    let mut best:u128 = 0;
    for i in 6..39 {
        let mut total_elapsed:Duration = Duration::ZERO;
        for j in 0..3 {
            let start = Instant::now();
            let stones = part3(100, i);
            let elapsed = start.elapsed();
            total_elapsed += elapsed;
        }

        if total_elapsed/3 < cur_time {
            best = i;
        }
        println!("{}, {:?}", i, total_elapsed/3);
    }
    println!("Best: {}", best);
}

fn part1(blinks: u8) -> usize {
    let mut stones:Vec<u128> = INPUT.trim()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    for _i in 0..blinks {
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
    return stones.len();
}

fn get_num_digits(x:&u128) -> u128 {
    return x.to_string().len().try_into().unwrap();
}

fn split_digit(x:&u128) -> (u128,u128) {
    let string = x.to_string();
    let string_len:usize = string.len().try_into().unwrap();
    let half_size = (string_len/2).try_into().unwrap();
    let split_string = string.split_at(half_size);
    return (split_string.0.parse::<u128>().unwrap(),
            split_string.1.parse::<u128>().unwrap());
}

fn add_stone(stones:&mut HashMap<u128,u128>, stone:u128, cur_value:u128) {
    stones.entry(stone).or_default();
    let cur_count:u128 = *stones.get(&stone).unwrap();
    stones.insert(stone, cur_count+cur_value);
}

fn part2(blinks: u8) -> u128 {
    let mut stones:HashMap<u128,u128> = HashMap::<u128,u128>::new();
    INPUT.trim()
        .split(" ")
        .for_each(|x| {
            let num = x.parse::<u128>().unwrap();
            add_stone(&mut stones, num, 1);
        });
    for _i in 0..blinks {
        let mut new_stones:HashMap<u128,u128> = HashMap::<u128,u128>::new();
        for (key, value) in &stones {
            if *key == 0 {
                add_stone(&mut new_stones, 1, *value);
            }
            else if get_num_digits(key)%2 == 0 {
                let new_digits = split_digit(key);
                add_stone(&mut new_stones, new_digits.0, *value);
                add_stone(&mut new_stones, new_digits.1, *value);
            }
            else {
                add_stone(&mut new_stones, *key*2024, *value);
            }
        }
        stones = new_stones;
        // println!("{:?}", stones);
    }
    let mut total = 0;
    stones.into_iter().for_each(|(_key,value)| {
        total += value;
    });
    return total;
}

struct Result {
    is_split: bool,
    pebble_1: u128,
    pebble_2: u128
}

fn part3(blinks: u8, digits: u128) -> u128 {
    let mut stones:HashMap<u128,u128> = HashMap::<u128,u128>::new();
    let mut cache:HashMap<u128, Result> =  HashMap::<u128, Result>::new();
    let limit = 10 ^ digits;

    INPUT.trim()
        .split(" ")
        .for_each(|x| {
            let num = x.parse::<u128>().unwrap();
            add_stone(&mut stones, num, 1);
        });
    for _i in 0..blinks {
        let mut new_stones:HashMap<u128,u128> = HashMap::<u128,u128>::new();
        for (key, value) in &stones {
            if *key > limit {
                if cache.contains_key(key) {
                    let result: &Result = cache.get(&key).unwrap();
                    if result.is_split {
                        add_stone(&mut new_stones, result.pebble_1, *value);
                        add_stone(&mut new_stones, result.pebble_2, *value);
                    } else {
                        add_stone(&mut new_stones, result.pebble_1, *value);
                    }
                    continue;
                }
            }
            if *key == 0 {
                add_stone(&mut new_stones, 1, *value);
            }
            else if get_num_digits(key)%2 == 0 {
                let new_digits = split_digit(key);
                add_stone(&mut new_stones, new_digits.0, *value);
                add_stone(&mut new_stones, new_digits.1, *value);
                if *key > limit {
                    cache.insert(*key, Result{is_split: true, pebble_1: new_digits.0, pebble_2: new_digits.1});
                }
            }
            else {
                add_stone(&mut new_stones, *key*2024, *value);
            }
        }
        stones = new_stones;
        // println!("{:?}", stones);
    }
    let mut total = 0;
    stones.into_iter().for_each(|(_key,value)| {
        total += value;
    });
    return total;
}

fn just_mem(blinks: u8) -> usize {
    let mut stones:Vec<u128> = INPUT.trim()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    let mut cache:HashMap<u128, Result> =  HashMap::<u128, Result>::new();
    for _i in 0..blinks {
        let mut j = 0;
        while j < stones.len() {
            if cache.contains_key(&stones[j]) {
                let result: &Result = cache.get(&stones[j]).unwrap();
                if result.is_split {
                    stones[j] = result.pebble_1;
                    stones.insert(j+1, result.pebble_2);
                    j+=2;

                } else {
                    stones[j] = result.pebble_1;
                }
            }
            else if stones[j] == 0 {
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
    return stones.len();
}