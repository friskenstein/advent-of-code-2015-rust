use std::cmp::min;

fn part1(input: &str) -> i64 {
    // How many total square feet of wrapping paper should they order?
    input.lines().fold(0, |acc, present| acc + {
            let p: Vec<&str> = present.split('x').collect();
            let l = p[0].parse::<i64>().unwrap();
            let w = p[1].parse::<i64>().unwrap();
            let h = p[2].parse::<i64>().unwrap();
            2*l*w + 2*w*h + 2*h*l + min(min(l*w, w*h), h*l)
        }
    )
}

fn part2(input: &str) -> i64 {
    // How many total feet of ribbon should they order?
    input.lines().fold(0, |acc, present| acc + {
            let p: Vec<&str> = present.split('x').collect();
            let l = p[0].parse::<i64>().unwrap();
            let w = p[1].parse::<i64>().unwrap();
            let h = p[2].parse::<i64>().unwrap();
            w * h * l + min(min(l+l+w+w, w+w+h+h), h+h+l+l)
        }
    )
}

fn main() {
	let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one() {
		assert_eq!(part1("2x3x4".into()), 58);
		assert_eq!(part1("1x1x10".into()), 43);
		assert_eq!(part1("2x3x4\n1x1x10".into()), 58+43);
	}

	#[test]
	fn two() {
		assert_eq!(part2("2x3x4".into()), 34);
		assert_eq!(part2("1x1x10".into()), 14);
		assert_eq!(part2("2x3x4\n1x1x10".into()), 34+14);
	}
}
