#[derive(PartialEq)]
enum Mode {
    Unknown,
    Increasing,
    Decreasing
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut mode: Mode = Mode::Unknown;
    let mut safe: bool = true;
    for i in 1..nums.len() {
        let difference: i32 = nums[i] - nums[i - 1];
        if difference.abs() > 3 || difference == 0 {
            safe = false;
            break;
        }

        if difference > 0 {
            match mode {
                Mode::Unknown => mode = Mode::Increasing,
                Mode::Decreasing => {
                    safe = false;
                    break;
                }
                _ => {}
            }
        } else {
            match mode {
                Mode::Unknown => mode = Mode::Decreasing,
                Mode::Increasing => {
                    safe = false;
                    break;
                }
                _ => {}
            }
        }
    }
    return safe;
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut count: i32 = 0;
    for line in input {
        let nums: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if is_safe(&nums) {
            count += 1;
        }
    }
    return count;
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut count: i32 = 0;
    for line in input {
        let nums: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if is_safe(&nums) {
            count += 1;
            continue;
        }
        for ignored_idx in 0..nums.len() {
            let filtered_nums = nums.iter()
                .enumerate()
                .filter(|(i, _)| *i as i32 != ignored_idx as i32)
                .map(|(_, val)| *val)
                .collect();
            if is_safe(&filtered_nums) {
                count += 1;
                break;
            }
        }
    }
    return count;
}
        
