const INPUT: &str = include_str!(".././input");
use std::collections::HashSet;

// right, down, left, up 
const DX: [i32;4] = [1, 0, -1, 0]; // col modifier
const DY: [i32;4] = [0, 1, 0, -1]; // row modifier

fn main() {
    let grid:Vec<Vec<char>> = INPUT.trim()
        .split("\n")
        .map(|row| row.trim().chars().collect())
        .collect();
    println!("{:?}", grid);

    let mut regions:Vec<HashSet<(usize,usize)>> = vec![];
    let mut explored:HashSet<(usize,usize)> = HashSet::new();
    let mut score_1 = 0;
    let mut score_2 = 0;

    grid.clone().into_iter().enumerate().for_each(|(row_i, row)| {
        row.into_iter().enumerate().for_each(|(col_i, col)| {
            if !explored.contains(&(row_i,col_i)) {
                let new_region = get_region(&col, &grid, &(row_i,col_i), &mut HashSet::new());
                explored.extend(&new_region);
                println!("{:?}", new_region);
                let perimeter = calculate_perimeter(&new_region, grid.len(), grid[0].len());
                let sides = calculate_sides(&col, &new_region, &grid, grid.len(), grid[0].len());
                println!("Area: {}, Perimeter: {}, Sides: {}", new_region.len(), perimeter, sides);
                score_1 += new_region.len() as i32 * perimeter;
                score_2 += sides * new_region.len() as i32;
                regions.push(new_region);
            }
        })
    });
    println!("{}", score_1);
    println!("{}", score_2);
}

fn get_region<'a>(ch:& char, map:& Vec<Vec<char>>, location:& (usize,usize), region:&'a mut HashSet<(usize,usize)>) -> HashSet<(usize,usize)> {
    region.insert(*location);
    for i in 0..4 {
        let row_i = DY[i] + location.0 as i32;
        let col_i = DX[i] + location.1 as i32;
        let row = row_i as usize;
        let col = col_i as usize;
        if (row <= map.len() - 1) &
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

// number of corners
// don't ask
fn calculate_sides(ch: &char, region: &HashSet<(usize,usize)>, map:& Vec<Vec<char>>, width: usize, height: usize) -> i32 {
    // right, down, left, up 
    let mut num_corners = 0;

    if region.len() == 1 {
        return 4;
    }

    for location in region {
        let mut adjacency: [bool; 4] = [false, false, false, false];
        for i in 0..4 {
            let row_i = DY[i] + location.0 as i32;
            let col_i = DX[i] + location.1 as i32;
            let row = row_i as usize;
            let col = col_i as usize;
            if (row <= width - 1) &
                (col <= height - 1) {

                if region.contains(&(row, col)) {
                    adjacency[i] = true;
                }
            }
        }
        let num_adjacent = adjacency.into_iter().filter(|b| *b).count();
        if num_adjacent == 1 {
            num_corners += 2
        }
        // Square Corners
        else if num_adjacent == 2 {
            if adjacency[1] & adjacency[0] { // down right
                num_corners += 1; 
                if map[location.0+1][location.1+1] != *ch { // not filled
                    num_corners+=1;
                }
            }
            else if adjacency[1] & adjacency[2] { // down left
                num_corners += 1;
                if map[location.0+1][location.1-1] != *ch { // not filled
                    num_corners+=1;
                }
            }
            else if adjacency[3] & adjacency[2] { // up left
                num_corners += 1;
                if map[location.0-1][location.1-1] != *ch { // not filled
                    num_corners+=1;
                }
            }
            else if adjacency[3] & adjacency[0] { // up right
                num_corners += 1;
                if map[location.0-1][location.1+1] != *ch { // not filled
                    num_corners+=1;
                }
            }
        }
        // T Corners
        // // right, down, left, up 
        else if num_adjacent == 3 {
            // Left, Right, Up
            if adjacency[2] & adjacency[0] & adjacency[3] {
                // Not filled TL TR
                if (map[location.0-1][location.1-1] != *ch) {
                    num_corners += 1;
                }
                if (map[location.0-1][location.1+1] != *ch) {
                    num_corners += 1;
                }
            }
            // Left, Right, Down
            if adjacency[2] & adjacency[0] & adjacency[1] {
                // Not filled BL BR
                if (map[location.0+1][location.1-1] != *ch) {
                    num_corners += 1;
                }
                if (map[location.0+1][location.1+1] != *ch) {
                    num_corners += 1;
                }
            }
            // Up, Down, Left
            if adjacency[3] & adjacency[1] & adjacency[2] {
                // Not filled TL BL
                if (map[location.0-1][location.1-1] != *ch) {
                    num_corners += 1;
                }
                if (map[location.0+1][location.1-1] != *ch) {
                    num_corners += 1;
                }
            }
            // Up, Down, Right
            if adjacency[3] & adjacency[1] & adjacency[0] {
                // Not filled TR BR
                if (map[location.0-1][location.1+1] != *ch) {
                    num_corners += 1;
                }
                if (map[location.0+1][location.1+1] != *ch) {
                    num_corners += 1;
                }
            }
        }
        else if num_adjacent == 4 {
            let changes: Vec<(i32,i32)> = vec![(1,1), (1,-1),(-1,-1),(-1,1)];
            for change in changes {
                if map[(location.0 as i32 + change.0) as usize]
                    [(location.1 as i32 + change.1) as usize] != *ch {
                    num_corners += 1;
                }
            }
        }
        println!("{:?} {}", location, num_corners);
    }
    return num_corners;
}