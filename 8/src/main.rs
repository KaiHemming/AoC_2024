const INPUT: &str = include_str!(".././input");
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
   part1();
}
fn part1() {
    let rows:Vec<&str> = INPUT.lines().collect();
    let mut nodes: HashMap<char,Vec<(usize,usize)>> = HashMap::<char,Vec<(usize,usize)>>::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();
    
    let height = rows.len();
    let width = rows[0].len();

    println!("{} by {}", height, width);

    for row_i in 0..height {
        let row:Vec<char> = rows[row_i].chars().collect();
        for col_i in 0..width {
            if row[col_i] != ".".chars().next().expect("") {
                if nodes.contains_key(&row[col_i]) {
                    nodes.get_mut(&row[col_i]).expect("").push((row_i,col_i));
                } else {
                    nodes.insert(row[col_i], vec![(row_i, col_i)]);
                }
            }
        }
    }
    println!("{:?}", nodes);

    for freq in nodes {
        let mut permutations: HashSet<((usize,usize),(usize,usize))> = HashSet::<((usize,usize),(usize,usize))>::new();
        // let mut freq_antinodes: HashSet<(i32,i32)> = HashSet::<(i32,i32)>::new();
        for node in &freq.1 {
            for compare_node in &freq.1 {
                if node != compare_node {
                    if permutations.contains(&(*node, *compare_node)) {
                        continue;
                    }
                    let dist = (node.0 as i32 - compare_node.0 as i32, node.1 as i32 - compare_node.1 as i32);
                    let antinode_1 = (node.0 as i32 + dist.0, node.1 as i32 + dist.1);
                    let antinode_2 = (compare_node.0 as i32 - dist.0 as i32, compare_node.1 as i32 - dist.1 as i32);
                    if check_in_bounds(&antinode_1, &height, &width) {
                        // freq_antinodes.insert(antinode_1);
                        antinodes.insert(antinode_1);
                    }
                    if check_in_bounds(&antinode_2, &height, &width) {
                        // freq_antinodes.insert(antinode_2);
                        antinodes.insert(antinode_2);
                    }
                    permutations.insert((*node,*compare_node));
                    permutations.insert((*compare_node,*node));
                }
            }
        }
        // let mut freq_antinodes_vec = Vec::from_iter(freq_antinodes);
        // antinodes.extend(freq_antinodes_vec);
    }
    println!("{:?}", antinodes);
    println!("{}", antinodes.len());
}

fn check_in_bounds(location:&(i32,i32), height: &usize, width: &usize) -> bool {
    if (location.0 >= 0) & 
        (location.0 <= *height as i32 - 1) &
        (location.1 >= 0) &
        (location.1 <= *width as i32 - 1) {
            return true;
    } 
    return false;
}


fn part2() {
    let rows:Vec<&str> = INPUT.lines().collect();
    let mut nodes: HashMap<char,Vec<(usize,usize)>> = HashMap::<char,Vec<(usize,usize)>>::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();
    // let mut antinodes: Vec<(i32,i32)> = vec![];
    
    let height = rows.len();
    let width = rows[0].len();

    println!("{} by {}", height, width);

    for row_i in 0..height {
        let row:Vec<char> = rows[row_i].chars().collect();
        for col_i in 0..width {
            if row[col_i] != ".".chars().next().expect("") {
                if nodes.contains_key(&row[col_i]) {
                    nodes.get_mut(&row[col_i]).expect("").push((row_i,col_i));
                } else {
                    nodes.insert(row[col_i], vec![(row_i, col_i)]);
                }
            }
        }
    }
    println!("{:?}", nodes);

    for freq in nodes {
        let mut permutations: HashSet<((usize,usize),(usize,usize))> = HashSet::<((usize,usize),(usize,usize))>::new();
        for node in &freq.1 {
            for compare_node in &freq.1 {
                if node != compare_node {
                    if permutations.contains(&(*node, *compare_node)) {
                        continue;
                    }
                    let dist = (node.0 as i32 - compare_node.0 as i32, node.1 as i32 - compare_node.1 as i32);
                    let mut antinode_1 = (node.0 as i32 + dist.0, node.1 as i32 + dist.1);
                    let mut antinode_2 = (compare_node.0 as i32 - dist.0 as i32, compare_node.1 as i32 - dist.1 as i32);
                    while check_in_bounds(&antinode_1, &height, &width) {
                        antinodes.insert(antinode_1);
                        // antinodes.push(antinode_1);
                        antinode_1 = (antinode_1.0 + dist.0, antinode_1.1 + dist.1);
                    }
                    while check_in_bounds(&antinode_2, &height, &width) {
                        antinodes.insert(antinode_2);
                        // antinodes.push(antinode_2);
                        antinode_2 = (antinode_2.0 - dist.0, antinode_2.1 - dist.1);
                    }
                    permutations.insert((*node,*compare_node));
                    permutations.insert((*compare_node,*node));
                }
            }
            antinodes.insert((node.0 as i32, node.1 as i32));
        }
    }
    // for row_i in 0..height  {
    //     for col_i in 0..width {
    //         let row:Vec<char> = rows[row_i].chars().collect();
    //         if antinodes.contains(&(row_i as i32, col_i as i32)) {
    //             print!("#");
    //         }
    //         else {
    //             print!("{}", row[col_i]);
    //         }
    //     }
    //     println!();
    // }
    println!("{:?}", antinodes);
    println!("{}", antinodes.len());
}
