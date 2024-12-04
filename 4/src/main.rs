const INPUT: &str = include_str!(".././input");

// Part1: mul\(\d{1,3},\d{1,3}\)
// Part2: (mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))

fn main() {
    part2();
}

fn part1() {
    let rows:Vec<&str> = INPUT.trim().split("\n").collect();
    let mut word_search:Vec<Vec<&str>> = vec![];
    for i in 0..rows.len() {
        word_search.push(Vec::new());
        let cols:Vec<&str> = rows[i].trim().split("").collect();
        for j in 0..cols.len() {
            if (cols[j].len() > 0) {
                word_search[i].push(cols[j]);
            }
        }
    }
    let mut hits = 0;
    for rows in 0..word_search.len() {
        for cols in 0..word_search[0].len() {
            if word_search[rows][cols] != "X" {
                continue;
            }
            // // Left to Right
            if cols <= word_search[0].len()-4 {
                if (word_search[rows][cols+1] == "M")
                    & (word_search[rows][cols+2]=="A")
                    & (word_search[rows][cols+3]=="S") {
                        println!("{}, {} LR", cols, rows);
                        hits += 1;
                    }
            }
            // // Right to Left
            if cols >= 3 {
                if (word_search[rows][cols-1] == "M")
                    & (word_search[rows][cols-2] == "A")
                    & (word_search[rows][cols-3] == "S") {
                        println!("{}, {} RL", cols, rows);
                        hits += 1;
                }
            }
            // Vertical
            // // T to B
            if rows <= word_search.len()-4 {
                if (word_search[rows+1][cols] == "M")
                    & (word_search[rows+2][cols] == "A")
                    & (word_search[rows+3][cols] == "S") {
                        println!("{}, {} TB", cols, rows);
                        hits += 1;
                    }
            }
            // // B to T
            if rows >= 3 {
                if (word_search[rows-1][cols] == "M") 
                    & (word_search[rows-2][cols] == "A")
                    & (word_search[rows-3][cols] == "S") {
                        println!("{}, {} BT", cols, rows);
                        hits += 1;
                    }
            }

            // Diagonal
            // // TR to BL
            if (cols >= 3) & (rows <= word_search[0].len()-4) {
                if (word_search[rows+1][cols-1] == "M") 
                    & (word_search[rows+2][cols-2] == "A")
                    & (word_search[rows+3][cols-3] == "S") {
                        println!("{}, {} TRBL", cols, rows);
                        hits += 1;
                    }
            }

            // // TL to BR
            if (cols <= word_search[0].len()-4) & (rows <= word_search[0].len()-4) {
                if (word_search[rows+1][cols+1] == "M") 
                    & (word_search[rows+2][cols+2] == "A")
                    & (word_search[rows+3][cols+3] == "S") {
                        println!("{}, {} TLBR", cols, rows);
                        hits += 1;
                    }
            }

            // // BR to TL
            if (rows >= 3) & (cols >= 3) {
                if (word_search[rows-1][cols-1] == "M")
                & (word_search[rows-2][cols-2] == "A")
                & (word_search[rows-3][cols-3] == "S") {
                    println!("{}, {} BRTL", cols, rows);
                    hits += 1;
                }
            }

            // // BL to TR
            if (rows >= 3) & (cols <= word_search[0].len()-4) {
                if (word_search[rows-1][cols+1] == "M")
                & (word_search[rows-2][cols+2] == "A")
                & (word_search[rows-3][cols+3] == "S") {
                    println!("{}, {} BLTR", cols, rows);
                    hits += 1;
                }
            }
        }
    }
    println!("{} hits", hits);
}

// Must be 2 ms and ss, they must be adjacent.
fn part2() {
    let rows:Vec<&str> = INPUT.trim().split("\n").collect();
    let mut word_search:Vec<Vec<&str>> = vec![];
    for i in 0..rows.len() {
        word_search.push(Vec::new());
        let cols:Vec<&str> = rows[i].trim().split("").collect();
        for j in 0..cols.len() {
            if (cols[j].len() > 0) {
                word_search[i].push(cols[j]);
            }
        }
    }
    let mut hits = 0;
    for rows in 0..word_search.len() {
        for cols in 0..word_search[0].len() {
            if word_search[rows][cols] != "A" {
                continue;
            }
            if (rows < 1) | (rows > word_search.len()-2) | (cols < 1) | (cols > word_search.len()-2){
                continue;
            }
            // TL, TR
            // BL, BR
            let vals:Vec<Vec<&str>> = vec![vec![word_search[rows-1][cols-1], 
                                                word_search[rows-1][cols+1]],
                                            vec![word_search[rows+1][cols-1],
                                                word_search[rows+1][cols+1]]];
            let mut is_valid = true;
            let mut num_s = 0;
            for val_rows in 0..vals.len() {
                for val_cols in 0..vals[0].len() {
                    if vals[val_rows][val_cols] == "S" {
                        num_s += 1
                    }
                    if (vals[val_rows][val_cols] == "X") | (vals[val_rows][val_cols] == "A") {
                        is_valid = false;
                    }
                    // one not in same row or one not in same column
                    let mut other_row = 0;
                    let mut other_col = 0;
                    if val_rows == 0 {
                        other_row = 1;
                    }
                    if val_cols == 0 {
                        other_col == 1;
                    }
                    if (vals[val_rows][val_cols] == "M") & !((vals[other_row][val_cols] == "M") | (vals[val_rows][other_col] == "M")) {
                        is_valid = false;
                    }
                }
            }
            if num_s != 2 {
                is_valid = false;
            }
            if is_valid {
                hits += 1;
            }
        }
    }
    println!("{} hits", hits);
}