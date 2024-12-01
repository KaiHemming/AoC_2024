use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut vec1 = Vec::<i32>::new();
    let mut vec2 = Vec::<i32>::new();
    let mut right_list_freq: HashMap<i32,i32> = HashMap::new();

    // Read from file
    let f= File::open("./input").unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        let unwrapped_line = line.unwrap();
        let mut vals = unwrapped_line.split("   ");
        let val_left = vals.next().unwrap().parse::<i32>().unwrap();
        let val_right = vals.next().unwrap().parse::<i32>().unwrap();
        if !right_list_freq.contains_key(&val_right) {
            right_list_freq.insert(val_right, 1);
        } else {
            right_list_freq.insert(val_right, 1 + right_list_freq[&val_right]);
        }
        vec1.push(val_left);
        vec2.push(val_right);
    }

    vec1.sort();
    vec2.sort();

    let mut tot_diff = 0;
    let mut similarity_score = 0;

    for i in 0..vec1.len() {
        let diff_val = vec1[i] - vec2[i];
        tot_diff += i32::abs(diff_val);

        let mut similarity_val = 0;
        if right_list_freq.contains_key(&vec1[i]) {
            similarity_val = vec1[i] * right_list_freq[&vec1[i]];
        } 

        similarity_score += similarity_val;
    }

    println!("Part 1: {}", tot_diff);
    println!("Part 2: {}", similarity_score);
}
