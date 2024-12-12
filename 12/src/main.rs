const INPUT: &str = include_str!(".././input");
use std::collections::HashSet;

const DX: [i32;4] = [1, 0, -1, 0];
const DY: [i32;4] = [0, 1, 0, -1];

fn main() {
    part1();
}

fn get_region<'a>(ch:& char, map:& Vec<Vec<char>>, location:& (usize,usize), region:&'a mut HashSet<(usize,usize)>) -> HashSet<(usize,usize)> {
    region.insert(*location);
    for i in 0..4 {
        let row_i = DY[i] + location.0 as i32;
        let col_i = DX[i] + location.1 as i32;
        let row = row_i as usize;
        let col = col_i as usize;
        if (row >= 0) & 
            (row <= map.len() - 1) &
            (col >= 0) &
            (col <= map[0].len() - 1) {

            if &map[row][col] == ch {
                if region.contains(&(row,col)) {
                    continue;
                }
                region.insert((row, col));
                get_region(ch, map, &(row, col), region);
            }
        } 
    }
    return region.clone();
}

fn calculate_perimeter(region: &HashSet<(usize,usize)>, width: usize, height: usize) -> i32 {
    let mut total_perimeter = 0;
    for location in region {
        let mut local_perimeter = 0;
        for i in 0..4 {
            let row_i = DY[i] + location.0 as i32;
            let col_i = DX[i] + location.1 as i32;
            let row = row_i as usize;
            let col = col_i as usize;
            if (row <= width - 1) &
                (col <= height - 1) {

                if !region.contains(&(row, col)) {
                    local_perimeter += 1;
                }
            }
            else {
                local_perimeter += 1;
            }
        }
        total_perimeter += local_perimeter;
    }
    return total_perimeter;
}

fn part1() {
    let grid:Vec<Vec<char>> = INPUT.trim()
        .split("\n")
        .map(|row| row.trim().chars().collect())
        .collect();
    println!("{:?}", grid);

    let mut regions:Vec<HashSet<(usize,usize)>> = vec![];
    let mut explored:HashSet<(usize,usize)> = HashSet::new();
    let mut score = 0;

    grid.clone().into_iter().enumerate().for_each(|(row_i, row)| {
        row.into_iter().enumerate().for_each(|(col_i, col)| {
            if !explored.contains(&(row_i,col_i)) {
                let new_region = get_region(&col, &grid, &(row_i,col_i), &mut HashSet::new());
                explored.extend(&new_region);
                println!("{:?}", new_region);
                println!("Area: {}, Perimeter: {}", new_region.len(), calculate_perimeter(&new_region, grid.len(), grid[0].len()));
                score += new_region.len() as i32 * calculate_perimeter(&new_region, grid.len(), grid[0].len());
                regions.push(new_region);
            }
        })
    });
    println!("{}", score);
}

fn part2() {
    
}
