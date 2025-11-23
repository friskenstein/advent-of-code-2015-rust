fn main() {
	let input = include_str!("../input.txt").trim();
	println!("Part 1: {}", part1(input));
	println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
	let mut result = input.to_string();
	for _ in 0..40 {
		result = process(&result);
	}
	result.len()
}

fn part2(input: &str) -> usize {
    let mut cur = input.to_string();
    let mut next = String::new();

    for _ in 0..50 {
        next.clear();
        process_into(&cur, &mut next);
        std::mem::swap(&mut cur, &mut next);
    }

    cur.len()
}

fn process_into(input: &str, out: &mut String) {
    let bytes = input.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        let current = bytes[i];
        let mut count = 1;
        i += 1;

        while i < len && bytes[i] == current {
            count += 1;
            i += 1;
        }

        out.push_str(&count.to_string());
        out.push(current as char);
    }
}

fn process(input: &str) -> String {
    let mut out = String::new();
    process_into(input, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("1"), "11");
        assert_eq!(process("11"), "21");
        assert_eq!(process("21"), "1211");
        assert_eq!(process("1211"), "111221");
        assert_eq!(process("111221"), "312211");
    }
}
