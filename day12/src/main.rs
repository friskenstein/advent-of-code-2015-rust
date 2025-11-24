fn sum_integers(s: &str) -> i64 {
    let mut sum = 0i64;
    let mut current = 0i64;
    let mut sign = 1i64;
    let mut in_number = false;

    for ch in s.chars() {
        match ch {
            '-' if !in_number => {
                // start of a signed number
                sign = -1;
                current = 0;
                in_number = true;
            }
            '0'..='9' => {
                if !in_number {
                    // start of an unsigned number
                    in_number = true;
                    sign = 1;
                    current = ch as i64 - '0' as i64;
                } else {
                    current = current * 10 + (ch as i64 - '0' as i64);
                }
            }
            _ => {
                if in_number {
                    sum += sign * current;
                    in_number = false;
                    current = 0;
                    sign = 1;
                }
            }
        }
    }

    // if string ended in a number, flush it
    if in_number {
        sum += sign * current;
    }

    sum
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", sum_integers(input));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sum_integers_works() {
		assert_eq!(sum_integers("[1,2,3]"), 6);
		assert_eq!(sum_integers(r#"{"a":2,"b":4}"#), 6);
		assert_eq!(sum_integers("[[[3]]]"), 3);
		assert_eq!(sum_integers(r#"{"a":{"b":4},"c":-1}"#), 3);
		assert_eq!(sum_integers(r#"{"a":[-1,1]}"#), 0);
		assert_eq!(sum_integers(r#"[-1,{"a":1}]"#), 0);
		assert_eq!(sum_integers("[]"), 0);
		assert_eq!(sum_integers("{}"), 0);
	}
}
