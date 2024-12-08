const INPUT: &str = include_str!(".././input");

fn main() {
   part2();
}

fn part1() {
    let rows:Vec<&str> = INPUT.trim().lines().collect();
    let mut data: Vec<(i64,Vec<i64>)> = vec![];
    for line in rows {
        let goal_sequence_str: Vec<&str> = line.split(":").collect();
        let goal:i64 = goal_sequence_str[0].parse::<i64>().unwrap();
        let sequence:Vec<i64> = goal_sequence_str[1].trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        data.push((goal,sequence));
    }
    let mut total = 0;
    // println!("{:#?}", data);
    for entry in &data {
        println!("Testing entry {:?}", entry);
        let goal = entry.0;
        if can_get_result(0, &entry.1, 0, goal, "") {
            println!("Success!");
            total += goal;
        }
    }
    println!("{}", total);
}

fn can_get_result(i:usize, sequence:&Vec<i64>, total: i64, goal: i64, steps:&str) -> bool {
    if (total == goal) & (i == sequence.len()) {
        return true;
    }
    if (total > goal) | (i > sequence.len()-1) {
        return false;
    }

    if i == 0 {
        if can_get_result(i+1, sequence, total + sequence[i], goal, &(steps.to_owned() + "+")) | 
            can_get_result(i+1, sequence, 1 * sequence[i], goal, &(steps.to_owned() + "*")) {
            return true;
    }
    }
    else {
        if can_get_result(i+1, sequence, total + sequence[i], goal, &(steps.to_owned() + "+")) | 
            can_get_result(i+1, sequence, total * sequence[i], goal, &(steps.to_owned() + "*")) {
            return true;
        }
    }
    return false;
}

fn can_get_result_with_concat(i:usize, sequence:&Vec<i64>, total: i64, goal: i64, steps:&str) -> bool {
    if (total == goal) & (i == sequence.len()) {
        return true;
    }
    if (total > goal) | (i > sequence.len()-1) {
        return false;
    }

    if i == 0 {
        if can_get_result_with_concat(i+1, sequence, (total.to_string() + &sequence[i].to_string()).parse().expect(""), goal, &(steps.to_owned() + "||")) |
            can_get_result_with_concat(i+1, sequence, total + sequence[i], goal, &(steps.to_owned() + "+")) | 
            can_get_result_with_concat(i+1, sequence, 1 * sequence[i], goal, &(steps.to_owned() + "*")) {
            return true;
    }
    }
    else {
        if can_get_result_with_concat(i+1, sequence, (total.to_string() + &sequence[i].to_string()).parse().expect(""), goal, &(steps.to_owned() + "||")) |
            can_get_result_with_concat(i+1, sequence, total + sequence[i], goal, &(steps.to_owned() + "+")) | 
            can_get_result_with_concat(i+1, sequence, total * sequence[i], goal, &(steps.to_owned() + "*")) {
            return true;
        }
    }
    return false;
}

fn part2() {
    let rows:Vec<&str> = INPUT.trim().lines().collect();
    let mut data: Vec<(i64,Vec<i64>)> = vec![];
    for line in rows {
        let goal_sequence_str: Vec<&str> = line.split(":").collect();
        let goal:i64 = goal_sequence_str[0].parse::<i64>().unwrap();
        let sequence:Vec<i64> = goal_sequence_str[1].trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        data.push((goal,sequence));
    }
    let mut total = 0;
    // println!("{:#?}", data);
    for entry in &data {
        println!("Testing entry {:?}", entry);
        let goal = entry.0;
        if can_get_result_with_concat(0, &entry.1, 0, goal, "") {
            println!("Success!");
            total += goal;
        }
    }
    println!("{}", total);
}
