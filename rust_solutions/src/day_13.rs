struct Machine {
    a_x: f64,
    a_y: f64,
    b_x: f64,
    b_y: f64,
    p_x: f64,
    p_y: f64
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i64 {
    let mut machines: Vec<Machine> = vec![];
    for idx in 0..input.len() {
        if input[idx].is_empty() { continue; }
        if &input[idx][7..=7] == "B" { continue; }
        if &input[idx][0..=0] == "P" { continue; }
        let line_a = &input[idx];
        let line_b = &input[idx + 1];
        let line_p = &input[idx + 2];
        let a = line_a.strip_prefix("Button A: ").unwrap().split(", ").map(|x| x[2..x.len()].parse::<f64>().unwrap()).collect::<Vec<f64>>();
        let b = line_b.strip_prefix("Button B: ").unwrap().split(", ").map(|x| x[2..x.len()].parse::<f64>().unwrap()).collect::<Vec<f64>>();
        let p = line_p.strip_prefix("Prize: ").unwrap().split(", ").map(|x| x[2..x.len()].parse::<f64>().unwrap()).collect::<Vec<f64>>();
        machines.push(
            Machine {
                a_x: a[0],
                a_y: a[1],
                b_x: b[0],
                b_y: b[1],
                p_x: p[0],
                p_y: p[1]
            }
        );
    }

    let mut total: i64 = 0;

    for m in machines {
        let min_cost = brute_force(&m);
        if let Some(mc) = min_cost {
            total += mc;
        }
    }
    total
}

fn brute_force(m: &Machine) -> Option<i64> {
    let mut min_cost: Option<i64> = None;
    for b in 0..=100 {
        let a = (m.p_x - m.b_x * b as f64) / m.a_x;
        if a * m.a_y + b as f64 * m.b_y == m.p_y {
            if a.round() == a {
                let cost = (a * 3f64) as i64 + b;
                if let None = min_cost {
                    min_cost = Some(cost);            
                    break;
                }
            }
        }
    }
    min_cost
}


pub fn solve_b(input: &Vec<String>, debug: bool) -> i64 {
    let mut machines: Vec<Machine> = vec![];
    for idx in 0..input.len() {
        if input[idx].is_empty() { continue; }
        if &input[idx][7..=7] == "B" { continue; }
        if &input[idx][0..=0] == "P" { continue; }
        let line_a = &input[idx];
        let line_b = &input[idx + 1];
        let line_p = &input[idx + 2];
        let a = line_a.strip_prefix("Button A: ").unwrap().split(", ").map(|x| x[2..x.len()].parse::<f64>().unwrap()).collect::<Vec<f64>>();
        let b = line_b.strip_prefix("Button B: ").unwrap().split(", ").map(|x| x[2..x.len()].parse::<f64>().unwrap()).collect::<Vec<f64>>();
        let p = line_p.strip_prefix("Prize: ").unwrap().split(", ").map(|x| x[2..x.len()].parse::<f64>().unwrap() + 10000000000000f64).collect::<Vec<f64>>();
        machines.push(
            Machine {
                a_x: a[0],
                a_y: a[1],
                b_x: b[0],
                b_y: b[1],
                p_x: p[0],
                p_y: p[1]
            }
        );
    }

    let mut total: i64 = 0;

    for m in machines {
        let b = (m.p_y - (m.a_y * m.p_x) / (m.a_x)) / ((-m.b_x * m.a_y) / m.a_x + m.b_y);
        let a = (m.p_x - m.b_x * b) / m.a_x;

        let a_error = (a.round() - a).abs();
        let b_error = (b.round() - b).abs();
        if a_error > 0.001 || b_error > 0.001 { continue; }
        let answer = a.round() as i64 * 3i64 + b.round() as i64;
        total += answer;
    }

    total
}
        
