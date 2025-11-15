fn main() {
    let input = include_str!("../input.txt");
    println!("Ends up at floor {}", algo(input));
    println!("Enters basement at position {}", basement(input));
}

fn algo(input: &str) -> i32 {
    let mut floor = 0;
    for i in 0..input.len() {
        floor += match input.get(i..=i) {
            Some("(") => 1,
            Some(")") => -1,
            Some(_) => 0,
            None => 0,
        };
    }
    floor
}

fn basement(input: &str) -> i32 {
    let mut floor = 0;
    for i in 0..input.len() {
        floor += match input.get(i..=i) {
            Some("(") => 1,
            Some(")") => -1,
            Some(_) => 0,
            None => 0,
        };
        if floor == -1 {
            return i as i32 + 1
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(algo("(())"), 0);
        assert_eq!(algo("()()"), 0);
        assert_eq!(algo("((("), 3);
        assert_eq!(algo("(()(()("), 3);
        assert_eq!(algo("))((((("), 3);
        assert_eq!(algo("())"    ), -1);
        assert_eq!(algo("))("    ), -1);
        assert_eq!(algo(")))"    ), -3);
        assert_eq!(algo(")())())"), -3);
    }

    #[test]
    fn test_two() {
        assert_eq!(basement(")"), 1);
        assert_eq!(basement("()())"), 5);
    }
}
