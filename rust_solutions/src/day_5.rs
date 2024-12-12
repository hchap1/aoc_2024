use std::collections::{HashSet,HashMap,VecDeque};

fn check_validity(hash: &HashMap<i32,Vec<i32>>, sequence: &Vec<i32>) -> bool {
    let mut disqualified_numbers: HashSet<i32> = HashSet::new();
    for item in sequence {
        if disqualified_numbers.contains(item) {
            return false;
        }
        if let Some(pre) = hash.get(item) {
            for num in pre {
                disqualified_numbers.insert(*num);
            }
        }
    }
    true
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    // Numbers that need to come before a given number
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut p2_idx: usize = 0;
    for (idx, line) in input.iter().enumerate() {
        if line.is_empty() { p2_idx = idx; break; }
        let parts = line.split('|').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if let Some(pre) = hash.get_mut(&parts[1]) {
            pre.push(parts[0]);
        } else {
            hash.insert(parts[1], vec![parts[0]]);
        }
    }

    let mut total: i32 = 0;

    for idx in p2_idx+1..input.len() {
        let sequence: Vec<i32> = input[idx].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if !check_validity(&hash, &sequence) { continue; }
        total += sequence[(sequence.len() as f32 / 2f32).floor() as usize];
    }

    total
}

fn recursion(hash: &HashMap<i32,Vec<i32>>, target: i32, to_ignore: &mut HashSet<i32>) -> Vec<i32> {
    if let Some(pre) = hash.get(&target) {
        let mut prior_numbers: Vec<i32> = vec![];
        for num in pre {
            if to_ignore.contains(&num) { continue; }
            prior_numbers.push(*num);
            to_ignore.insert(*num);
            prior_numbers.append(&mut recursion(hash, *num, to_ignore));
        }
        return prior_numbers;
    }
    vec![]
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>) -> Result<Vec<i32>, String> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut all_nodes: HashSet<i32> = HashSet::new();
    
    // Build in-degree map and adjacency list
    for (&node, dependencies) in graph {
        all_nodes.insert(node);
        for &dep in dependencies {
            all_nodes.insert(dep);
            in_degree.entry(dep).and_modify(|e| *e += 1).or_insert(1);
        }
        adjacency_list.entry(node).or_default().extend(dependencies);
    }
    
    // Ensure all nodes are represented in in-degree map
    for &node in &all_nodes {
        in_degree.entry(node).or_insert(0);
    }
    
    // Initialize queue with nodes that have zero in-degree
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();
    
    let mut sorted_order = Vec::new();
    
    // Perform the topological sort
    while let Some(current) = queue.pop_front() {
        sorted_order.push(current);
        
        if let Some(neighbors) = adjacency_list.get(&current) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    
    // Check for cycles (incomplete sort)
    if sorted_order.len() != all_nodes.len() {
         return Err("Graph has a cycle; topological sort not possible.".to_string());
    }
    
    Ok(sorted_order)
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    // Numbers that need to come before a given number
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut p2_idx: usize = 0;
    for (idx, line) in input.iter().enumerate() {
        if line.is_empty() { p2_idx = idx; break; }
        let parts = line.split('|').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if let Some(pre) = hash.get_mut(&parts[1]) {
            pre.push(parts[0]);
        } else {
            hash.insert(parts[1], vec![parts[0]]);
        }
    }

    let mut total: i32 = 0;
    let order = topological_sort(&hash).unwrap();

    for idx in p2_idx+1..input.len() {
        let mut new_sequence: Vec<i32> = vec![0];
        let nums = input[idx].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if check_validity(&hash, &nums) { continue; }
        let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        for val in &order {
            if set.contains(val) {
                new_sequence.push(*val);
            }
        }
        println!("{new_sequence:?}");
        total += new_sequence[(new_sequence.len() as f32 / 2f32).floor() as usize];
    }

    total
}
       
