fn main() {
	let input = include_str!("../input.txt").trim();
	println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
	let mut result = input.to_owned();
	for _ in 0..40 {
		result = process(result.to_owned());
	}
	result.len()
}

fn process(input: String) -> String {
	let mut result = String::new();
    let mut chars = input.chars().peekable();

    if let Some(mut current) = chars.next() {
        let mut count = 1;

        for c in chars {
            if c == current {
                count += 1;
            } else {
                // Flush the current run
                result.push_str(&count.to_string());
                result.push(current);

                // Start a new run
                current = c;
                count = 1;
            }
        }

        // Flush the final run
        result.push_str(&count.to_string());
        result.push(current);
    }
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() {
		assert_eq!(process("1".to_owned()), "11".to_owned());
		assert_eq!(process("11".to_owned()), "21".to_owned());
		assert_eq!(process("21".to_owned()), "1211".to_owned());
		assert_eq!(process("1211".to_owned()), "111221".to_owned());
		assert_eq!(process("111221".to_owned()), "312211".to_owned());
	}
}
