use std::collections::HashSet;

struct Region {
    plots: Vec<(i32, i32)>,
    plot_type: char
}

#[derive(Clone, PartialEq, Eq, Copy, Hash)]
struct Fence {
    horizontal: bool,
    position: (i32, i32),
    above: bool
}

fn is_in_bounds(grid: &Vec<Vec<char>>, point: (usize, usize)) -> bool {
    point.0 < grid.len() && point.1 < grid[point.0].len()
}

fn recursively_destroy_region(grid: &mut Vec<Vec<char>>, point: (usize, usize), region: &mut Region) {
    region.plots.push((point.0 as i32, point.1 as i32));
    let plot_type: char = grid[point.0][point.1];
    grid[point.0][point.1] = '.';
    for r in -1i32..=1i32 {
        for c in -1i32..=1i32 {
            if (r + c).abs() != 1 {
                continue;
            }
            if r == -1 && point.0 == 0 { continue; }
            if c == -1 && point.1 == 0 { continue; }
            let new_point: (usize, usize) = ((point.0 as i32 + r) as usize, (point.1 as i32 + c) as usize);
            if !is_in_bounds(grid, new_point) { continue; }
            if grid[new_point.0][new_point.1] == plot_type {
                recursively_destroy_region(grid, new_point, region);
            }
        }
    }
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut regions: Vec<Region> = vec![];

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] != '.' {
                let mut region: Region = Region { plots: vec![], plot_type: grid[r][c] };
                recursively_destroy_region(&mut grid, (r, c), &mut region);
                regions.push(region);
            }
        }
    }

    let mut result: i32 = 0;

    for region in regions {
        let area = region.plots.len() as i32;
        let mut perimeter = 0i32;

        let positions: HashSet<(i32, i32)> = HashSet::from_iter(region.plots.clone().into_iter());

        for plot in region.plots {
            if plot.0 > 0 {
                if !positions.contains(&(plot.0 - 1, plot.1)) {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }

            if plot.1 > 0 {
                if !positions.contains(&(plot.0, plot.1 - 1)) {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }

            if !positions.contains(&(plot.0 + 1, plot.1)) {
                perimeter += 1;
            }

            if !positions.contains(&(plot.0, plot.1 + 1)) {
                perimeter += 1;
            }
        }

        if debug {
            println!("Region {} has perimeter = {perimeter}, and area = {area}", region.plot_type);
        }

        result += area * perimeter;
    }

    result
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    if debug {
        println!();
    }
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut regions: Vec<Region> = vec![];

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] != '.' {
                let mut region: Region = Region { plots: vec![], plot_type: grid[r][c] };
                recursively_destroy_region(&mut grid, (r, c), &mut region);
                regions.push(region);
            }
        }
    }

    let mut result: i32 = 0;

    for region in regions {
        let area = region.plots.len() as i32;
        let mut perimeter = 0i32;
        let mut fences: Vec<Fence> = vec![];

        let positions: HashSet<(i32, i32)> = HashSet::from_iter(region.plots.clone().into_iter());

        for plot in &region.plots {
            if !positions.contains(&(plot.0 - 1, plot.1)) {
                fences.push(Fence { horizontal: true, position: (plot.0 - 1, plot.1), above: true });
            }
            if !positions.contains(&(plot.0 + 1, plot.1)) {
                fences.push(Fence { horizontal: true, position: (plot.0 + 1, plot.1), above: false });
            }
            if !positions.contains(&(plot.0, plot.1 - 1)) {
                fences.push(Fence { horizontal: false, position: (plot.0, plot.1 - 1), above: true });
            }
            if !positions.contains(&(plot.0, plot.1 + 1)) {
                fences.push(Fence { horizontal: false, position: (plot.0, plot.1 + 1), above: false });
            }
        }
        let fence_set: HashSet<Fence> = HashSet::from_iter(fences.clone().into_iter());
        for plot in region.plots {
            if !positions.contains(&(plot.0 - 1, plot.1)) {
                if !fence_set.contains(&Fence{ horizontal: true, position: (plot.0 - 1, plot.1 - 1), above: true }) {
                    perimeter += 1;
                }
            }

            if !positions.contains(&(plot.0, plot.1 - 1)) {
                if !fence_set.contains(&Fence{ horizontal: false, position: (plot.0 - 1, plot.1 - 1), above: true }) {
                    perimeter += 1;
                }
            }

            if !positions.contains(&(plot.0 + 1, plot.1)) {
                if !fence_set.contains(&Fence{ horizontal: true, position: (plot.0 + 1, plot.1 - 1), above: false }) {
                    perimeter += 1;
                }
            }

            if !positions.contains(&(plot.0, plot.1 + 1)) {
                if !fence_set.contains(&Fence{ horizontal: false, position: (plot.0 - 1, plot.1 + 1), above: false }) {
                    perimeter += 1;
                }
            }
        }

        if debug {
            println!("Region {} has perimeter = {perimeter}, and area = {area}", region.plot_type);
        }

        result += area * perimeter;
    }

    result
}
        
