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
    travelled_locations: HashSet<Location>
}

fn build_guard(location: Location) -> Guard {
    Guard {
        facing: Direction::UP,
        location: location,
        travelled_locations: HashSet::new()
    }
}

impl Guard {
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
        self.location = copy_location(&map[new_coord.0 as usize][new_coord.1 as usize]);
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
   part1();
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
    println!("{}", guard.travelled_locations.len());
}

fn part2() {
}
