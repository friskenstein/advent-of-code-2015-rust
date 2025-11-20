use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> i32 {
    let mut cities_set: HashSet<&str> = HashSet::new();
    let mut dist: HashMap<(&str, &str), i32> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut it = line.split_whitespace();
        let from = it.next().unwrap();
        it.next();
        let to = it.next().unwrap();
        it.next();
        let d: i32 = it.next().unwrap().parse().unwrap();

        cities_set.insert(from);
        cities_set.insert(to);

        dist.insert((from, to), d);
        dist.insert((to, from), d);
    }

    let cities: Vec<&str> = cities_set.into_iter().collect();
    let n = cities.len();

    let mut best = i32::MAX;
    let mut used = vec![false; n];
    let mut order = Vec::with_capacity(n);

    
    dfs(0, n, &cities, &dist, &mut used, &mut order, &mut best, 0);

    best
}

fn dfs<'a>(
    depth: usize,
    n: usize,
    cities: &Vec<&'a str>,
    dist: &HashMap<(&'a str, &'a str), i32>,
    used: &mut [bool],
    order: &mut Vec<usize>,
    best: &mut i32,
    current_len: i32,
) {
    // all cities chosen -> full path
    if depth == n {
        if current_len < *best {
            *best = current_len;
        }
        return;
    }

    for i in 0..n {
        if used[i] {
            continue;
        }

        // dist from previous city
        let extra = if let Some(&prev_idx) = order.last() {
            dist[&(cities[prev_idx], cities[i])]
        } else {
            0
        };

        let next_len = current_len + extra;

        // small pruning: if already worse than best, skip
        if next_len >= *best {
            continue;
        }

        used[i] = true;
        order.push(i);
        dfs(depth + 1, n, cities, dist, used, order, best, next_len);
        order.pop();
        used[i] = false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"), 605);
    }
}
