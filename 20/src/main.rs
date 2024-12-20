const INPUT: &str = include_str!(".././input");
use std::collections::HashMap;

fn main() {
    let mut map: Vec<Vec<bool>> = vec![];
    let mut track: Vec<(usize, usize)> = vec![];
    let mut start: (usize, usize) = (0,0);
    let mut end:(usize, usize) = (0,0);

    INPUT.lines().enumerate().for_each(|(row, line)| {
        let mut map_row: Vec<bool> = vec![];
        line.trim().split("").enumerate().for_each(|(col,x)| {
            if x != "" {
                if x == "." {
                    map_row.push(true);
                } else if x == "S" {
                    map_row.push(true);
                    start = (row, col-1);
                } else if x == "E" {
                    map_row.push(true);
                    end = (row, col-1);
                }
                else {
                    map_row.push(false);
                }
            }
        });
        map.push(map_row);
    });

    // println!("{:?}", map);

    let mut has_found_end = false;
    let mut prev_pos: (usize, usize) = start;
    let mut cur_pos = start;
    let movement: [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];
    println!("{:?}", start);
    track.push(start);

    // Create track
    while !has_found_end {
        for m in movement {
            let row = (cur_pos.0 as i32 + m.0) as usize;
            let col = (cur_pos.1 as i32 + m.1) as usize;

            if row > map.len() { continue; }
            if col > map[0].len() { continue; }

            if (row as usize, col as usize) == end  {
                has_found_end = true;
                track.push(end);
                break;
            } 
            if (map[row][col]) & 
                ((row,col) != cur_pos) &
                ((row,col) != prev_pos)  {
                    prev_pos = cur_pos;
                    cur_pos = (row,col);
                    track.push((row,col));
                    break;
            }
        }
    }
    
    println!("{:?}", track);
    println!("{} picoseconds", track.len()-1);

    let cheat_movements: [(i32, i32); 4] = [(0,2), (0,-2), (2,0), (-2,0)];
    let mut cheats: HashMap<usize, usize> = HashMap::new();
    for i in 0..track.len()-3 {
        for cheat_movement in cheat_movements {
            let pos = track[i];
            let row = (pos.0 as i32 + cheat_movement.0) as usize;
            let col = (pos.1 as i32 + cheat_movement.1) as usize;

            if row > map.len() { continue; }
            if col > map[0].len() { continue; }

            // let search = track[i+1..track.len()-1].binary_search(&(row,col));
            let search = track[i+3..track.len()].iter().position(|x| x == &(row,col));
            if search.is_some() {
                let picoseconds_saved = search.unwrap()+1;
                if cheats.contains_key(&picoseconds_saved) {
                    println!("Pos: {:?} Cheat Pos: {:?} Saved: {:?}", pos, (row,col), picoseconds_saved);
                    cheats.insert(picoseconds_saved, cheats.get(&picoseconds_saved).unwrap() + 1);
                } else {
                    println!("Pos: {:?} Cheat Pos: {:?} Saved: {:?}", pos, (row,col), picoseconds_saved);
                    cheats.insert(picoseconds_saved, 1);
                }
            }
        }
    }
    let mut num_cheats_greaterorequal_100 = 0;
    println!("{:?}", cheats);
    for cheat in cheats {
        println!("Picoseconds: {} found cheat(s): {}", cheat.0, cheat.1);
        if cheat.0 >= 100 {
            num_cheats_greaterorequal_100 += cheat.1;
        }
    }
    println!("{}", num_cheats_greaterorequal_100);
}
