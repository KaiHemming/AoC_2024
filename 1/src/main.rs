use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut vec1 = Vec::<i32>::new();
    let mut vec2 = Vec::<i32>::new();

    // Read from file
    let f= File::open("./input").unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        let unwrapped_line = line.unwrap();
        let mut vals = unwrapped_line.split("   ");
        vec1.push(vals.next().unwrap().parse::<i32>().unwrap());
        vec2.push(vals.next().unwrap().parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();
    let mut tot_diff = 0;

    for i in 0..vec1.len() {
        let val = vec1[i] - vec2[i];
        tot_diff += i32::abs(val);
    }

    println!("{}", tot_diff);
}
