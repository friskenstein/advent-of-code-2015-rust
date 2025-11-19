fn main() {
    println!("Part 1: {}", part1(include_str!("../input.txt")));
}

fn num_of_chars(input: &str) -> (i32, i32) {
    let literal = input.len() as i32;
    let mut memory = 0;
    
    let mut i = 1; // skip first quote    
    let bytes = input.as_bytes();
    let end = bytes.len() - 1; // index of last quote

    while i < end {
        if bytes[i] == b'\\' {
            match bytes[i + 1] {
                b'\\' | b'"' => {memory += 1; i += 2},
                b'x' => {memory += 1; i += 4;},
                _ => {memory += 1; i += 2;},
            }
        } else {
            memory += 1;
            i += 1;
        }
    }

    (literal, memory)
}

fn diff_oper(input: &str) -> i32 {
    let op = num_of_chars(input);
    op.0 - op.1
}

fn part1(input: &str) -> i32 {
    input.split_whitespace().fold(0, |acc, s| acc + diff_oper(s) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_of_chars() {
        assert_eq!(num_of_chars(r#""""#), (2, 0));
        assert_eq!(num_of_chars(r#""abc""#), (5, 3));
        assert_eq!(num_of_chars(r#""aaa\"aaa""#), (10, 7));
        assert_eq!(num_of_chars(r#""\x27""#), (6, 1));
    }


    #[test]
    fn test_part1() {
        assert_eq!(part1(r#"""
"abc"
"aaa\"aaa"
"\x27""#), 12);
    }

}
