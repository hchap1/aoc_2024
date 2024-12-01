use std::collections::HashMap;

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];
    for l in input {
        let s = l.split("  ").map(|x| x.to_string()).collect::<Vec<String>>();
        if debug {
            println!("{s:?}");
        }
        list_a.push(s.get(0).unwrap().parse::<i32>().unwrap());
        list_b.push(s.get(1).unwrap().strip_prefix(' ').unwrap().parse::<i32>().unwrap());
    }
    list_a.sort();
    list_b.sort();
    let mut dif: i32 = 0;
    for i in 0..list_a.len() {
        dif += (list_a[i] - list_b[i]).abs();
    }
    return dif;
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut list: Vec<i32> = vec![];
    for l in input {
        let s = l.split("  ").map(|x| x.to_string()).collect::<Vec<String>>();
        if debug {
            println!("{s:?}");
        }
        list.push(s.get(0).unwrap().parse::<i32>().unwrap());
        let n = s.get(1).unwrap().strip_prefix(' ').unwrap().parse::<i32>().unwrap();
        if let Some(count) = counts.get_mut(&n) {
            *count += 1;
        } else {
            counts.insert(n, 1);
        }
    }
    let mut sim_score: i32 = 0;
    for num in list {
        let count = match counts.get(&num) {
            Some(count) => count,
            None => &0
        };
        sim_score += num * count;
    }
    sim_score
}
        
