const INPUT: &str = include_str!(".././input");
use regex::Regex;

//167 too low
fn main() {
    part1();
}

fn part1() {
    let separator = Regex::new(r"\n[\s*$]").unwrap();

    let mut longest_rule = 0;
    let mut input= separator.split(INPUT);
    let rules: Vec<String> = input.next().unwrap().trim().split(",").map(|rule| {
        if rule.len() > longest_rule {
            longest_rule = rule.len();
        }
        ["^",rule.trim(),"$"].join("")
    }).collect();
    // println!("{:?}", rules);


    let patterns = input.next().unwrap().trim().split("\n")
        .map(|entry| {
            entry.trim().chars().collect()
    }).collect::<Vec<Vec<char>>>();

    let mut valid_patterns = 0;
    for pattern in patterns {
        let mut combo: Vec<String> = vec![];
        // println!("Checking pattern {:?}", pattern);
        let find_combo = find_combination(&rules, &pattern, 0, &mut combo, &mut String::from(""), longest_rule);
        // println!("{:?}", find_combo);
        if find_combo.is_some() {
            valid_patterns+=1;
        }
    }
    println!("{}", valid_patterns);
}

fn find_combination(rules: &Vec<String>, 
    input: &Vec<char>, 
    cur_pos: usize, 
    combo: &mut Vec<String>,
    backlog: &mut String,
    longest_rule: usize) -> Option<Vec<String>> {

        if cur_pos >= input.len(){
            if (backlog.len() <= 0) {
                return Some(combo.to_vec());
            } else {
                return None;
            }
        }


        backlog.push(input[cur_pos]);
        // println!("Combo {:?}, backlog {}", combo, backlog);
        for rule in rules {
            let re = Regex::new(rule).unwrap();
            // println!("{}", rule);
            if re.is_match(backlog) {
                combo.push(rule.to_string());
                let find_combo = find_combination(rules, input, cur_pos+1, combo, &mut String::from(""), longest_rule);
                if find_combo.is_some() {
                    return find_combo;
                }
            }
        }
        if backlog.len() < longest_rule {
            return find_combination(rules, input, cur_pos+1, combo, backlog, longest_rule);
        }
        return None;

}

fn part2() {
}