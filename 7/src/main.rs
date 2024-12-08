const INPUT: &str = include_str!(".././testinput");

fn main() {
   part1();
}

// 1289579147154 too high
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
        if can_get_result(0, &entry.1, 0, goal) {
            println!("Success!");
            total += goal;
        }
    }
    println!("{}", total);
}

fn can_get_result(i:usize, sequence:&Vec<i64>, total: i64, goal: i64) -> bool {
    if total == goal {
        return true;
    }
    if (total > goal) | (i > sequence.len()-1) {
        return false;
    }

    if can_get_result(i+1, sequence, total + sequence[i], goal) | 
        can_get_result(i+1, sequence, total * sequence[i], goal){
        return true;
    }

    return false;
}

fn part2() {
}
