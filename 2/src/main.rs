const INPUT: &str = include_str!("../input");

fn main() {
    let mut total_valid = 0;

    for line in INPUT.lines() {
        let vals:Vec<i32> = line.split(" ").map(|x| x.parse()).flatten().collect();

        if vals[0] == vals[1] || i32::abs(vals[0] - vals[1]) > 3{
            continue;
        }

        let mut is_less_than = true;
        if vals[0] < vals[1] {
            is_less_than = false;
        }

        let mut prev_val = vals[1];
        let mut is_acs_or_desc = true;

        for i in 2..vals.len() {
            if i32::abs(prev_val - vals[i]) > 3 {
                is_acs_or_desc = false;
                break;
            }
            if is_less_than {
                if prev_val <= vals[i] {
                    is_acs_or_desc = false;
                    break;
                }
            } else {
                if prev_val >= vals[i] {
                    is_acs_or_desc = false;
                    break;
                }
            }
            prev_val = vals[i];
        }
        if is_acs_or_desc {
            total_valid += 1;
        }
    }
    println!("{}", total_valid);
}
