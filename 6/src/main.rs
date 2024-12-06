const INPUT: &str = include_str!(".././input.txt");

use std::collections::HashSet;

enum Direction {
    UP,
    DOWN, 
    LEFT,
    RIGHT
}

#[derive(Eq, Hash, PartialEq)]
pub struct Location {
    row: usize,
    col: usize,
    has_obstacle: bool
}

fn build_location(row: usize, col:usize, string_representation:&str) -> Location {
    Location {
        row: row,
        col: col,
        has_obstacle: string_representation == "#"
    }
}

fn copy_location(location: &Location) -> Location {
    Location {
        row: location.row,
        col: location.col,
        has_obstacle: location.has_obstacle
    }
}

pub struct Guard {
    facing: Direction,
    location: Location,
    travelled_locations: HashSet<Location>,
    retraversal_first_location: Option<Location>,
    has_looped: bool
}

fn build_guard(location: Location) -> Guard {
    Guard {
        facing: Direction::UP,
        location: copy_location(&location),
        travelled_locations: HashSet::from([copy_location(&location)]),
        retraversal_first_location: None,
        has_looped: false
    }
}

impl Guard {
    pub fn reset<'a>(&'a mut self, start_location: &Location) {
        self.facing = Direction::UP;
        self.location = copy_location(start_location);
        self.travelled_locations = HashSet::new();
        self.retraversal_first_location = None;
        self.has_looped = false;
    }
    pub fn do_movement<'a>(&'a mut self, map: &Vec<Vec<Location>>) -> bool {
        let cur_coord:(&usize, &usize) = (&self.location.row, &self.location.col);
        self.travelled_locations.insert(copy_location(&map[*cur_coord.0][*cur_coord.1]));

        // Get new coordinates
        let movement_vector:&(i32,i32) = &self.get_movement_vector();
        let new_coord:(i32, i32) = (self.location.row as i32 + movement_vector.0, 
                                    self.location.col as i32 + movement_vector.1);

        // println!("{}", new_coord.0 < 0);
        // println!("{}", new_coord.0 as usize > map.len()-1);
        // println!("{}", new_coord.1 < 0);
        // println!("{}", new_coord.1 as usize > map[0].len()-1);
        // Check if at edge
        if (new_coord.0 < 0) | 
            (new_coord.0 as usize > map.len()-1) |
            (new_coord.1 < 0) |
            (new_coord.1 as usize > map[0].len()-1) {

            return false;
        }

        // Check if need to turn
        if map[new_coord.0 as usize][new_coord.1 as usize].has_obstacle {
            if matches!(&self.facing, Direction::UP) {
                self.facing = Direction::RIGHT;
            }
            else if matches!(&self.facing, Direction::DOWN) {
                self.facing = Direction::LEFT;
            }
            else if matches!(&self.facing, Direction::LEFT) {
                self.facing = Direction::UP;
            }
            else if matches!(&self.facing, Direction::RIGHT) {
                self.facing = Direction::DOWN;
            }
            return self.do_movement(map);
        }
        // TODO: Prevent looping movement
        let new_location = &map[new_coord.0 as usize][new_coord.1 as usize];
        // println!("{}", self.travelled_locations.len());
        if self.travelled_locations.contains(new_location) {
            if self.retraversal_first_location.is_some() {
                // let thingy= self.retraversal_first_location.as_mut().expect("");
                if &map[self.retraversal_first_location.as_mut().expect("").row][self.retraversal_first_location.as_mut().expect("").col] == new_location {
                    // println!("Detected loop");
                    self.has_looped = true;
                    return false;
                }
            } else {
                // println!("Set first location of retraversal");
                self.retraversal_first_location = Some(copy_location(new_location));
            }
            // println!("Beep");
            // println!("{}", self.path_memory.len() >= self.travelled_locations.len());
            // self.retraversal_first_location.insert(copy_location(new_location));
            // if self.retraversal_first_location.len() >= self.travelled_locations.len() {
            //     self.has_looped = true;
            //     return false;
            // }
        } else {
            // println!("New location!");
            self.retraversal_first_location = None;
        }

        self.location = copy_location(new_location);
        return true;
    }
    // row, col
    pub fn get_movement_vector(&self) -> (i32,i32) {
        if matches!(&self.facing, Direction::UP) {
            return (-1, 0);
        }
        else if matches!(&self.facing, Direction::DOWN) {
            return (1, 0);
        }
        else if matches!(&self.facing, Direction::LEFT) {
            return (0, -1);
        }
        else if matches!(&self.facing, Direction::RIGHT) {
            return (0, 1);
        }
        return (0,0);
    }
}

fn print_board(map: &Vec<Vec<Location>>, guard: &Guard) {
    for row in map {
        for location in row {
            if location.has_obstacle {
                print!("#");
            } else if guard.location == *location {
                print!("G");
            } else if guard.travelled_locations.contains(location) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
   part2();
}

// . = free
// # = obstacle
fn part1() {
    let rows:Vec<&str> = INPUT.trim().lines().collect();
    let mut map:Vec<Vec<Location>> = vec![];
    let mut guard_option:Option<Guard> = None;
    for row_i in 0..rows.len() {
        let row:Vec<&str> = rows[row_i].split("").collect();
        let mut row_as_locations:Vec<Location> = vec![];
        for col_i in 0..row.len() {
            if row[col_i] == "^" {
                guard_option = Some(build_guard(build_location(row_i, col_i, row[col_i])));
            }
            row_as_locations.push(build_location(row_i, col_i, row[col_i]));
        }
        map.push(row_as_locations);
    }
    let mut guard:Guard = guard_option.expect("");
    while guard.do_movement(&map) {
        // print_board(&map, &guard);
        // println!();
    }
    // println!("{}", guard.travelled_locations.len());
}

fn part2() {
    let rows:Vec<&str> = INPUT.trim().lines().collect();
    let mut map:Vec<Vec<Location>> = vec![];
    let mut guard_option:Option<Guard> = None;
    for row_i in 0..rows.len() {
        let row:Vec<&str> = rows[row_i].split("").collect();
        let mut row_as_locations:Vec<Location> = vec![];
        for col_i in 0..row.len() {
            if row[col_i] == "^" {
                guard_option = Some(build_guard(build_location(row_i, col_i, row[col_i])));
            }
            row_as_locations.push(build_location(row_i, col_i, row[col_i]));
        }
        map.push(row_as_locations);
    }

    let mut guard:Guard = guard_option.expect("");
    let guard_start_location = copy_location(&guard.location);
    let mut num_looped = 0;

    for row_i in 0..rows.len() {
        for col_i in 0..rows[0].len() {
            if map[row_i][col_i].has_obstacle {
                continue;
            }
            map[row_i][col_i].has_obstacle = true;

            guard.reset(&guard_start_location);
            // println!("Placing guard");
            while guard.do_movement(&map) {
                // print_board(&map, &guard);
                // println!();
            }
            if guard.has_looped {
                // println!("Found loop.");
                num_looped += 1;
            } else {
                // println!("No loop.");
            }
            map[row_i][col_i].has_obstacle = false;
        }
    }
    println!("{}", num_looped);
}
