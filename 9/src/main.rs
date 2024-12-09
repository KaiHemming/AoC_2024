const INPUT: &str = include_str!(".././input");

fn main() {
   part1();
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
}
