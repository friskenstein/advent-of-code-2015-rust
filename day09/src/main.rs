use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let (cities, dist) = parse(input);
    let n = cities.len();

    let mut best = i32::MAX;
    let mut used = vec![false; n];
    let mut order = Vec::with_capacity(n);
    
    dfs(0, n, &cities, &dist, &mut used, &mut order, &mut best, true);

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
    find_min: bool,
) {
    if depth == n {

        let mut current_len = 0;
        for w in order.windows(2) {
            let a = &cities[w[0]];
            let b = &cities[w[1]];
            current_len += dist[&(a.as_ref(), b.as_ref())];
        }

        if find_min {
            if current_len < *best {
                *best = current_len;
            }
        } else {
            if current_len > *best {
                *best = current_len;
            }
        }
        return;
    }

    for i in 0..n {
        if used[i] {
            continue;
        }

        used[i] = true;
        order.push(i);
        dfs(depth + 1, n, cities, dist, used, order, best, find_min);
        order.pop();
        used[i] = false;
    }
}

fn part2(input: &str) -> i32 {
    let (cities, dist) = parse(input);
    let n = cities.len();

    let mut best = i32::MIN;
    let mut used = vec![false; n];
    let mut order = Vec::with_capacity(n);

    dfs(0, n, &cities, &dist, &mut used, &mut order, &mut best, false);

    best
}

fn parse(input: &str) -> (Vec<&str>, HashMap<(&str, &str),i32>) {
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
    
    (cities, dist)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2("London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"), 982);
    }
}
