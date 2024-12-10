const INPUT: &str = include_str!(".././input");
use std::collections::HashSet;

fn main() {
   part1();
}
// 220 too low
fn part1() {
    let map:Vec<Vec<u32>> = INPUT.trim().split("\n").map(|row| {
        row.trim().chars().map (|x| {
            x.to_digit(10).unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();
    // println!("{:?}", map);
    
    let mut trailheads:HashSet::<(usize,usize)> = HashSet::<(usize,usize)>::new();
    map.clone().into_iter()
        .enumerate()
        .for_each(|(row,r)| {
            r
            .into_iter()
            .enumerate()
            .for_each(|(col, x)| {
                if x == 0 {
                    // println!("NEW {}: {}, {}", x, row, col);
                    if traverse((row,col), &map) {
                        trailheads.insert((row,col));
                    }
                }
            })
        });
    
    println!("{}", trailheads.len());
}

fn traverse(pos: (usize,usize), map: &Vec<Vec<u32>>) -> bool {
    let cur_num = map[pos.0][pos.1];
    // println!("{}: {:?}", cur_num, pos);
    // check reached end of trail
    if cur_num == 9 {
        // println!("Found end");
        return true;
    }
    // check up
    if pos.0 > 0 {
        if map[pos.0-1][pos.1] == cur_num+1 {
            // println!("Up");
            if traverse((pos.0-1, pos.1), map) {
                return true;
            }
        }
    }
    // check down
    if pos.0 < map[0].len()-1 {
        if map[pos.0+1][pos.1] == cur_num+1 {
            // println!("Down");
            if traverse((pos.0+1, pos.1), map) {
                return true;
            }
        }
    }
    // check left
    if pos.1 > 0 {
        if map[pos.0][pos.1-1] == cur_num+1 {
            // println!("Left");
            if traverse((pos.0, pos.1-1), map) {
                return true;
            }
        }
    }
    // check right
    if pos.1 < map.len()-1 {
        if map[pos.0][pos.1+1] == cur_num+1 {
            // println!("Right");
            if traverse((pos.0, pos.1+1), map) {
                return true;
            }
        }
    }
    return false;
}

fn part2() {

}
