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
    let mut part_1_total:u32 = 0;
    let mut part_2_total:u32 = 0;
    map.clone().into_iter()
        .enumerate()
        .for_each(|(row,r)| {
            r
            .into_iter()
            .enumerate()
            .for_each(|(col, x)| {
                if x == 0 {
                    let results = traverse((row,col), &map);
                    part_1_total += results.0 as u32;
                    part_2_total += results.1 as u32;
                }
            })
        });
    println!("{}", part_1_total);
    println!("{}", part_2_total);
}

fn traverse(pos: (usize,usize), map: &Vec<Vec<u32>>) -> (usize, usize) {
    let mut incomplete_journeys: Vec<Vec<(usize,usize)>> = vec![vec![pos]];
    let mut complete_journeys: Vec<Vec<(usize,usize)>> = vec![];
    let mut unique_destinations: HashSet<(usize,usize)> = HashSet::<(usize,usize)>::new();
    while incomplete_journeys.len() > 0 {
        let first_journey: Vec<(usize,usize)> = incomplete_journeys[0].clone();
        let cur_pos = first_journey[first_journey.len()-1];
        let cur_num = map[cur_pos.0][cur_pos.1];

        // check reached end of trail
        if cur_num == 9 {
            complete_journeys.push(first_journey.clone());
            incomplete_journeys.remove(0 as usize);
            unique_destinations.insert(cur_pos);
            continue;
        }
        // check up
        if cur_pos.0 > 0 {
            if map[cur_pos.0-1][cur_pos.1] == cur_num+1 {
                let mut new_journey = first_journey.clone();
                new_journey.push((cur_pos.0-1, cur_pos.1));
                incomplete_journeys.push(new_journey);
            }
        }
        // check down
        if cur_pos.0 < map[0].len()-1 {
            if map[cur_pos.0+1][cur_pos.1] == cur_num+1 {
                let mut new_journey = first_journey.clone();
                new_journey.push((cur_pos.0+1, cur_pos.1));
                incomplete_journeys.push(new_journey);
            }
        }
        // check left
        if cur_pos.1 > 0 {
            if map[cur_pos.0][cur_pos.1-1] == cur_num+1 {
                let mut new_journey = first_journey.clone();
                new_journey.push((cur_pos.0, cur_pos.1-1));
                incomplete_journeys.push(new_journey);
            }
        }
        // check right
        if cur_pos.1 < map.len()-1 {
            if map[cur_pos.0][cur_pos.1+1] == cur_num+1 {
                let mut new_journey = first_journey.clone();
                new_journey.push((cur_pos.0, cur_pos.1+1));
                incomplete_journeys.push(new_journey);
            }
        }
        incomplete_journeys.remove(0 as usize);
    }
    return (unique_destinations.len(),complete_journeys.len());
}

fn part2() {

}
