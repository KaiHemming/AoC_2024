const INPUT: &str = include_str!(".././input");
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;
use fxhash::{FxBuildHasher, FxHashMap};

fn main() {
    // find_best_def_expensive();

    // println!("{}",just_mem(2));

    for i in 0..255 {
        let start = Instant::now();
    //     // let stones = part2(i);
    //     // let stones = part3(i, 170141183460469231731687303715884105728); //Most significant bit
    //     // let stones = part3(i, 85070591730234615865843651857942052864); // Second most significant bit
    //     // let stones = part3(i, 10^22);
            let stones = just_mem_recursive(i);
    //     let stones = just_mem(i, 10^22);
        let elapsed = start.elapsed();

        println!("{}, {:?}, {}", i, elapsed, stones);
    }

    println!("{}", just_mem_recursive(25));
}

// fn find_best_def_expensive() {
//     let mut cur_time: Duration = Duration::MAX;
//     let mut best:u8 = 0;
//     for i in 2..25 {
//         let mut total_elapsed:Duration = Duration::ZERO;
//         for j in 0..3 {
//             let start = Instant::now();
//             let stones = just_mem_recursive(75, i);
//             let elapsed = start.elapsed();
//             total_elapsed += elapsed;
//         }

//         if total_elapsed/3 < cur_time {
//             best = i;
//             cur_time = total_elapsed/3
//         }
//         println!("{}, {:?}", i, total_elapsed/3);
//     }
//     println!("Best: {}", best);
// }

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

fn part3(blinks: u8, limit: u128) -> u128 {
    let mut stones:HashMap<u128,u128> = HashMap::<u128,u128>::new();
    let mut cache:HashMap<u128, Result> =  HashMap::<u128, Result>::new();

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
                continue;
            }
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
                if get_num_digits(key)%2 == 0 {
                    let new_digits = split_digit(key);
                    add_stone(&mut new_stones, new_digits.0, *value);
                    add_stone(&mut new_stones, new_digits.1, *value);
                    cache.insert(*key, Result{is_split: true, pebble_1: new_digits.0, pebble_2: new_digits.1});
                    continue;
                }
                let new_stone = *key*2024;
                add_stone(&mut new_stones, new_stone, *value);
                cache.insert(*key, Result {is_split: false, pebble_1: new_stone, pebble_2:0});
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

fn just_mem(blinks: u8, limit: u128) -> usize {
    let mut stones:Vec<u128> = INPUT.trim()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    let mut cache:HashMap<u128, Result> =  HashMap::<u128, Result>::new();
    for _i in 0..blinks {
        let mut j = 0;
        while j < stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
                continue;
            }

            if stones[j] > limit {
                if cache.contains_key(&stones[j]) {
                    let result: &Result = cache.get(&stones[j]).unwrap();
                    if result.is_split {
                        stones[j] = result.pebble_1;
                        stones.insert(j+1, result.pebble_2);
                        j+=2;
                        continue;
                    } else {
                        stones[j] = result.pebble_1;
                    }
                }
                else if get_num_digits(&stones[j])%2 == 0 {
                    let key = stones[j];
                    let new_digits = split_digit(&stones[j]);
                    stones[j] = new_digits.0;
                    stones.insert(j+1, new_digits.1);
                    cache.insert(key, Result{is_split: true, pebble_1: new_digits.0, pebble_2: new_digits.1});
                    j += 2;
                    continue;
                }
                else {
                    let key = stones[j];
                    let mult = stones[j]*2024;
                    stones[j] = mult;
                    cache.insert(key, Result{is_split: false, pebble_1: mult, pebble_2: 0});
                }
            }

            else if get_num_digits(&stones[j])%2 == 0 {
                let key = stones[j];
                let new_digits = split_digit(&stones[j]);
                stones[j] = new_digits.0;
                stones.insert(j+1, new_digits.1);
                j += 2;
                continue;
            }
            else {
                let mult = stones[j]*2024;
                stones[j] = mult;
            }
            j += 1;
        }
        // println!("{:?}", stones);
    }
    // println!("{:?}", stones);
    return stones.len();
}

fn just_mem_recursive(blinks: u8) -> u128 {
    let mut stones:HashMap<u128,u128> = HashMap::<u128,u128>::new();
    let mut cache:HashMap<(u8, u128), u128> =  HashMap::<(u8, u128), u128>::new();

    INPUT.trim()
        .split(" ")
        .for_each(|x| {
            let num = x.parse::<u128>().unwrap();
            add_stone(&mut stones, num, 1);
        });

    let mut total: u128 = 0;
    for (key, value) in &stones {
        total += get_descendants(*key, *value, blinks, &mut cache);
    }
    return total;
}

fn get_descendants<'a>(stone: u128, num_stones: u128, iterations: u8, cache: &'a mut HashMap::<(u8, u128), u128>) -> u128 {
    if iterations <= 0 {
        cache.insert((iterations, stone), num_stones);
        return num_stones;
    }
    
    let cache_condition = iterations > 4;

    if cache_condition {
        if cache.contains_key(&(iterations, stone)) {
            return *cache.get(&(iterations, stone)).unwrap();
        }
    }

    let count:u128;

    if stone == 0 {
        count = get_descendants(1, num_stones, iterations-1, cache);
    }
    else if get_num_digits(&stone)%2 == 0 {
        let new_digits = split_digit(&stone);
        count = get_descendants(new_digits.0, num_stones, iterations-1, cache) +
            get_descendants(new_digits.1, num_stones, iterations-1, cache);
    }
    else {
        let new_stone = stone*2024;
        count = get_descendants(stone*2024, num_stones, iterations-1, cache);
    }
    if cache_condition {
        cache.insert((iterations, stone), count);
    }
    return count;
}
