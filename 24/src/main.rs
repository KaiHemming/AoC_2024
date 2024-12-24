const INPUT: &str = include_str!(".././input");
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let separator = Regex::new(r"\n[\s*$]").expect("");
    let start_vals_separator = Regex::new(r":\s").expect("");
    let word_separator = Regex::new(r"([\w]+)").expect("");
    let z_val_regex = Regex::new(r"^z\d+").expect("");

    let mut values: HashMap<&str, bool> = HashMap::<&str, bool>::new();
    let mut z_values: HashMap<&str, bool> = HashMap::<&str, bool>::new();
    let input: Vec<&str> = separator.split(INPUT).collect();

    input[0].lines().for_each(|line| {
        let captures: Vec<&str> = start_vals_separator.split(line.trim()).collect();
        let mut bool = false;
        if captures[1] == "1" {
            bool = true;
        }
        values.insert(captures[0], bool);
    });
    println!("{:?}", values);

    let mut equations: Vec<Vec<&str>> = vec![];
    input[1].lines().for_each(|line| {
        let mut equation: Vec<&str> = vec![];
        for (_, [word]) in word_separator.captures_iter(line).map(|c| c.extract()) {
            equation.push(word);
        }
        equations.push(equation);
    });
    println!("{:?}", equations);

    let mut i:usize = 0;
    while equations.len() > 0 {
        let equation = &equations[i];

        let remove_index = i;

        i += 1;
        if i >= equations.len() {
            i = 0;
        }

        if !(values.contains_key(equation[0]) &
            values.contains_key(equation[2])) {
                continue;
            }
        
        let bool_1 = values.get(equation[0]).unwrap();
        let bool_2 = values.get(equation[2]).unwrap();
        let mut output = false;
        match equation[1] {
            "AND" => {
                if bool_1 & bool_2 {
                    output = true;
                }
            },
            "OR" => {
                if bool_1 | bool_2 {
                    output = true;
                }
            },
            "XOR" => {
                if bool_1 ^ bool_2 {
                    output = true;
                }
            },
            &_ => {
                println!("BAH HUMBUG");
            }
        }
        // println!("Inserting {} {}", equation[3], output);
        values.insert(equation[3], output);
        if z_val_regex.is_match(equation[3]) {
            z_values.insert(equation[3], output);
        }
        equations.remove(remove_index);
        if i >= equations.len() {
            i = 0;
        }
    }
    let mut binary: String = String::from("");
    for i in (0..z_values.len()).rev() {
        let mut x = i.to_string();
        if i < 10 {
            x.insert(0, '0');
        }
        x.insert(0,'z');
        if *z_values.get(x.as_str()).expect("") {
            binary.push('1');
        } else {
            binary.push('0');
        }
    }
    println!("{}", binary);
    println!("{}", i64::from_str_radix(binary.as_str(), 2).expect(""));
}
