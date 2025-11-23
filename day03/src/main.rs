use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let (mut x, mut y) = (0, 0);
    map.insert(format!("{x},{y}"), 1);
    input.chars().for_each(|c| {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => (),
        }
        *map.entry(format!("{x},{y}")).or_default() += 1;
    });
    map.keys().count()
}

fn part2(input: &str) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let (mut x, mut y) = (0, 0);
    let (mut s, mut t) = (0, 0);
    let mut turn = 0;
    map.insert(format!("{x},{y}"), 1);
    input.chars().for_each(|c| {
        match (c, turn % 2) {
            ('>', 0) => x += 1,
            ('<', 0) => x -= 1,
            ('^', 0) => y += 1,
            ('v', 0) => y -= 1,
            ('>', 1) => s += 1,
            ('<', 1) => s -= 1,
            ('^', 1) => t += 1,
            ('v', 1) => t -= 1,
            (_, _) => (),
        }
        if turn % 2 == 0 {
            *map.entry(format!("{x},{y}")).or_default() += 1;
        } else {
            *map.entry(format!("{s},{t}")).or_default() += 1;
        }
        turn += 1;
    });
    map.keys().count()
}

pub fn main() {
	let input = include_str!("../input.txt");
    println!("Part 1 answer is: {}", part1(input));
    println!("Part 2 answer is: {}", part2(input));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one() {
		assert_eq!(part1(">".into()), 2);
		assert_eq!(part1("^>v<".into()), 4);
		assert_eq!(part1("^v^v^v^v^v".into()), 2);
	}
	#[test]
	fn two() {
		assert_eq!(part2("^v".into()), 3);
		assert_eq!(part2("^>v<".into()), 3);
		assert_eq!(part2("^v^v^v^v^v".into()), 11);
	}
}
