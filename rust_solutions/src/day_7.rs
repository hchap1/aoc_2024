fn num_to_operational_binary(num: i32, len: usize) -> Vec<char> {
    let binary: String = format!("{num:b}");
    let mut operators = binary.chars().map(|x| 
        match x {
            '0' => '+',
             _  => '*'
        }
    ).collect::<Vec<char>>();
    for _ in 0..(len - operators.len()) {
        operators.insert(0, '+');
    }
    operators
}

fn num_to_operational_trinary(mut num: i32, len: usize) -> Vec<usize> {
    let mut trinary: Vec<usize> = vec![0; len];
    for exponent in (0..len).rev() {
        while num > 3i32.pow(exponent as u32) {
            num -= 3i32.pow(exponent as u32);
            trinary[exponent] += 1;
        }
    }
    trinary
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i64 {
    let mut calibration_result: i64 = 0;
    for line in input {
        let mut found_target: bool = false;
        let mut nums: Vec<i64> = line.split(' ').map(|x| 
           match found_target {
                false => {
                    found_target = true;
                    x.strip_suffix(':').unwrap().parse::<i64>().unwrap()
                }
                _ => x.parse::<i64>().unwrap()
            } 
        ).collect::<Vec<i64>>();

        let target = nums.remove(0);
        let num_combinations: i32 = 2i32.pow((nums.len() - 1) as u32);

        for combination in 0..num_combinations {
            let mut result: i64 = nums[0];
            let operational_sequence: Vec<char> = num_to_operational_binary(combination, nums.len() - 1);

            for (idx, operator) in operational_sequence.iter().enumerate() {
                match operator {
                    '+' => result += nums[idx + 1],
                     _  => result *= nums[idx + 1]
                } 
            }

            if result == target {
                calibration_result += target;
                break
            }
        }
    }
    calibration_result
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i64 {
    let mut calibration_result: i64 = 0;
    for line in input {
        let mut found_target: bool = false;
        let mut nums: Vec<i64> = line.split(' ').map(|x| 
           match found_target {
                false => {
                    found_target = true;
                    x.strip_suffix(':').unwrap().parse::<i64>().unwrap()
                }
                _ => x.parse::<i64>().unwrap()
            } 
        ).collect::<Vec<i64>>();

        let target = nums.remove(0);
        let num_combinations: i32 = 3i32.pow((nums.len() - 1) as u32);

        if debug {
            println!("Numbers: {nums:?}");
        }

        for combination in 0..num_combinations {
            let mut result: i64 = nums[0];
            let operational_sequence: Vec<usize> = num_to_operational_trinary(combination + 1, nums.len() - 1);

            if debug {
                println!("Testing {}", operational_sequence.iter().map(|x| x.to_string()).collect::<String>());
            }

            for (idx, operator) in operational_sequence.iter().enumerate() {
                match operator {
                    0 => result += nums[idx + 1],
                    1 => result *= nums[idx + 1],
                    _ => {
                        result = (result.to_string() + &nums[idx + 1].to_string()).parse::<i64>().unwrap();
                    }
                } 
            }

            if result == target {
                calibration_result += target;
                break
            }
        }
    }
    calibration_result
}
        
