fn increment(pw: &mut [u8]) {
    // treat as base-26 number using ASCII 'a'..'z'
    let mut i = pw.len();
    while i > 0 {
        i -= 1;
        if pw[i] == b'z' {
            pw[i] = b'a';
        } else {
            pw[i] += 1;
            break;
        }
    }
}

fn skip_forbidden(pw: &mut [u8]) {
    // If we find i/o/l, bump it to the next letter and zero out the rest to 'a'
    for i in 0..pw.len() {
        match pw[i] {
            b'i' | b'o' | b'l' => {
                pw[i] += 1; // next char is always allowed (j, p, m)
                for j in (i + 1)..pw.len() {
                    pw[j] = b'a';
                }
                break;
            }
            _ => {}
        }
    }
}

fn has_straight(pw: &[u8]) -> bool {
    // Look for abc, bcd, ...
    for i in 0..pw.len().saturating_sub(2) {
        let a = pw[i];
        let b = pw[i + 1];
        let c = pw[i + 2];
        if b == a + 1 && c == a + 2 {
            return true;
        }
    }
    false
}

fn no_forbidden(pw: &[u8]) -> bool {
    !pw.iter().any(|&c| c == b'i' || c == b'o' || c == b'l')
}

fn has_two_pairs(pw: &[u8]) -> bool {
    use std::collections::HashSet;
    let mut pairs: HashSet<u8> = HashSet::new();
    let mut i = 0;
    while i + 1 < pw.len() {
        if pw[i] == pw[i + 1] {
            pairs.insert(pw[i]);
            i += 2; // skip overlapping
        } else {
            i += 1;
        }
    }
    pairs.len() >= 2
}

fn is_valid(pw: &[u8]) -> bool {
    has_straight(pw) && no_forbidden(pw) && has_two_pairs(pw)
}

fn next_password(start: &str) -> String {
    let mut pw = start.as_bytes().to_vec();
    // Clean up starting point if it already contains forbidden letters
    skip_forbidden(&mut pw);

    loop {
        increment(&mut pw);
        skip_forbidden(&mut pw);
        if is_valid(&pw) {
            return String::from_utf8(pw).unwrap();
        }
    }
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let next = next_password(input);
    println!("Part 1: {next}");
	let next2 = next_password(&next);
	println!("Part 2: {next2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_increment_works() {
        fn s(x: &str) -> String {
            let mut v = x.as_bytes().to_vec();
            increment(&mut v);
            String::from_utf8(v).unwrap()
        }

        assert_eq!(s("aaaaaaaa"), "aaaaaaab");
        assert_eq!(s("aaaaaaaz"), "aaaaaaba");
        assert_eq!(s("zzzzzzzz"), "aaaaaaaa");
        assert_eq!(s("xx"), "xy");
        assert_eq!(s("xy"), "xz");
        assert_eq!(s("xz"), "ya");
    }

    #[test]
    fn straight_detection() {
        assert!(has_straight(b"hijklmmn")); // hij
        assert!(has_straight(b"abcxyzab")); // abc and xyz
        assert!(!has_straight(b"abddffgj"));
    }

    #[test]
    fn forbidden_letters() {
        assert!(!no_forbidden(b"hijklmmn")); // has i and l
        assert!(no_forbidden(b"abcdffaa"));
    }

    #[test]
    fn two_pairs_detection() {
        assert!(has_two_pairs(b"abbceffg")); // bb, ff
        assert!(!has_two_pairs(b"abbcegjk")); // only bb
        assert!(!has_two_pairs(b"aaaa")); // only one distinct pair (a)
        assert!(has_two_pairs(b"aabb")); // aa, bb
    }

    #[test]
    fn validity_examples() {
        // examples from the description
        assert!(!is_valid(b"hijklmmn")); // forbidden letters
        assert!(!is_valid(b"abbceffg")); // no straight
        assert!(!is_valid(b"abbcegjk")); // not enough pairs
    }

    #[test]
    fn next_password_examples() {
        assert_eq!(next_password("abcdefgh"), "abcdffaa");
        assert_eq!(next_password("ghijklmn"), "ghjaabcc");
    }
}
