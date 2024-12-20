const INPUT: &str = include_str!(".././input");

fn main() {
   part2();
}
fn part1() {
    let mut last_file_num:i32 = 0;
    let mut disk:Vec<i32> = vec![];
    INPUT.trim().chars()
        .enumerate()
        .for_each(|(index, char)| {
            if index % 2 == 0 { // File
                for i in 0..char.to_digit(10).expect("") {
                    disk.push(last_file_num);
                }
                last_file_num += 1;
            } else {
                for i in 0..char.to_digit(10).expect("") {
                    disk.push(-1); // -1 represents free space
                }
            }
        });
    let mut i = 0;
    let mut j = disk.len()-1;
    // println!("{:?}", disk);
    while i != j {
        if disk[i] == -1 {
            while disk[j] == -1 {
                j -= 1;
            }
            disk[i] = disk[j];
            disk[j] = -1;
            j -= 1;
        }
        i += 1;
        // println!("{:?}", disk);
    }
    let mut total:i64 = 0;
    for k in 0..j+1 {
        total += disk[k] as i64 * k as i64;
    }
    println!("{}", total);
}


fn part2() {
    let mut last_file_num:i32 = 0;
    let mut disk:Vec<(i32,i32)> = vec![];
    INPUT.trim().chars()
        .enumerate()
        .for_each(|(index, char)| {
            if index % 2 == 0 { 
                disk.push((last_file_num, char.to_digit(10).expect("") as i32));
                last_file_num += 1;
            } else {
                disk.push((-1, char.to_digit(10).expect("") as i32));
            }
        });

    for j in (1..disk.len()).rev() {
        if disk[j].0 < 0 {
            continue;
        }
        // println!("{:?}", disk);
        for i in 0..j {
            let size_to_fill = disk[i].1;
            if (disk[i].0 < 0) & 
                (disk[j].1 <= size_to_fill) {
                // println!("moving {:?} to {:?}", disk[j], disk[i]);
                let size_remaining: i32 = size_to_fill - disk[j].1;
                disk[i] = disk[j];
                disk[j] = (-1, disk[i].1);

                if disk[j-1].0 < 0 {
                    disk[j].1 += disk[j-1].1;
                    disk.remove(j-1);
                    if j < disk.len() {
                        if disk[j].0 < 0 {
                            disk[j-1].1 += disk[j].1;
                            disk.remove(j);
                        }
                    }
                }
                else if j+1 < disk.len() {
                    if disk[j+1].0 < 0 {
                        disk[j].1 += disk[j+1].1;
                        disk.remove(j+1);
                    }
                }

                if size_remaining > 0 {
                    if disk[i+1].0 < 0 {
                        disk[i+1].1 += size_remaining;
                    } else {
                        disk.insert(i+1, (-1, size_remaining));
                    }
                }
                break;
            }
        }
    }

    let mut cur_index = 0;
    let mut total:i64 = 0;
    for pair in disk {
        if pair.0 != -1 {
            for i in 0..pair.1 {
                total += pair.0 as i64 * cur_index as i64;
                cur_index += 1;
            }
        }
        else {
            cur_index += pair.1;
        }
    }
    println!("{}", total);
}
