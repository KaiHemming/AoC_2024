const INPUT: &str = include_str!(".././input");

fn main() {
    part1();
}

fn part1() {
    let mut stones:Vec<u64> = INPUT.trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    for i in 0..75 {
        println!("{}", i);
        let mut j = 0;
        while j < stones.len() {
            if stones[j] == 0 {
                stones[j] = 1;
            }
            else if get_num_digits(&stones[j])%2 == 0 {
                let new_digits = split_digit(&stones[j]);
                stones[j] = new_digits.0;
                stones.insert(j+1, new_digits.1);
                j += 2;
                continue;
            }
            else {
                stones[j] = stones[j]*2024;
            }
            j += 1;
        }
    }
    // println!("{:?}", stones);
    println!("{}", stones.len());
}

fn get_num_digits(x:&u64) -> u64 {
    return x.to_string().len().try_into().unwrap();
}

fn split_digit(x:&u64) -> (u64,u64) {
    let string = x.to_string();
    let string_len:usize = string.len().try_into().unwrap();
    let half_size = (string_len/2).try_into().unwrap();
    let split_string = string.split_at(half_size);
    return (split_string.0.parse::<u64>().unwrap(),
            split_string.1.parse::<u64>().unwrap());
}

fn part2() {

}
