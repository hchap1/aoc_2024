
pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut count: i32 = 0;
    for line in input {
        for idx in 0..line.len() {
            if idx >= line.len() - 4 {
                break;
            }

            if &line[idx..idx+4].to_string() == "mul(" {
                if debug { println!("Mul detected."); }
                let mut dump: String = String::new();
                let slice = line[idx+4..line.len()].to_string();
                if debug { println!("Slice: {slice}."); }
                let mut iterator = slice.chars();
                let mut last_char: char = 'x';

                while last_char != ')' {

                    let chr = match iterator.next() {
                        Some(chr) => chr,
                        None => break
                    };

                    last_char = chr;
                    if chr != ')' { dump.push(chr); }
                }

                if debug { println!("Dump: {dump}"); }

                if last_char == ')' {
                    if !dump.contains(',') {
                        continue;
                    }

                    let split = dump.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
                    
                    if split.len() != 2 {
                        continue;
                    }

                    let a = &split[0];
                    let b = &split[1];

                    if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                        if debug { println!("Multiplied {a} x {b}."); }
                        count += a * b;
                    }
                }
            }
        }
    }

    count
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut count: i32 = 0;
    let mut enabled = true;
    for line in input {
        for idx in 0..line.len() {
            if idx >= line.len() - 4 {
                break;
            }

            if idx < line.len() - 7 {
                if &line[idx..idx+7].to_string() == "don't()" {
                    enabled = false;
                }
            }

            if &line[idx..idx+4].to_string() == "do()" {
                enabled = true;
            }

            if &line[idx..idx+4].to_string() == "mul(" && enabled {
                if debug { println!("Mul detected."); }
                let mut dump: String = String::new();
                let slice = line[idx+4..line.len()].to_string();
                if debug { println!("Slice: {slice}."); }
                let mut iterator = slice.chars();
                let mut last_char: char = 'x';

                while last_char != ')' {

                    let chr = match iterator.next() {
                        Some(chr) => chr,
                        None => break
                    };

                    last_char = chr;
                    if chr != ')' { dump.push(chr); }
                }

                if debug { println!("Dump: {dump}"); }

                if last_char == ')' {
                    if !dump.contains(',') {
                        continue;
                    }

                    let split = dump.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
                    
                    if split.len() != 2 {
                        continue;
                    }

                    let a = &split[0];
                    let b = &split[1];

                    if let (Ok(a), Ok(b)) = (a.parse::<i32>(), b.parse::<i32>()) {
                        if debug { println!("Multiplied {a} x {b}."); }
                        count += a * b;
                    }
                }
            }
        }
    }

    count
}
        
