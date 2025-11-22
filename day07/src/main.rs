use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Source {
	Const(u16),
	Wire(String),
}

#[derive(Clone, Debug)]
enum Expr {
	Direct(Source),
	Not(Source),
	And(Source, Source),
	Or(Source, Source),
	LShift(Source, u8),
	RShift(Source, u8),
}

type Circuit = HashMap<String, Expr>;

fn parse_source(s: &str) -> Source {
	if let Ok(v) = s.parse::<u16>() {
		Source::Const(v)
	} else {
		Source::Wire(s.to_string())
	}
}

fn parse_line(line: &str) -> (String, Expr) {
	let line = line.trim();
	if line.is_empty() {
		panic!("Empty line in input");
	}

	let mut parts = line.split(" -> ");
	let lhs = parts.next().expect("missing lhs");
	let rhs = parts.next().expect("missing rhs");
	if parts.next().is_some() {
		panic!("too many '->' in line: {line}");
	}

	let target = rhs.to_string();
	let tokens: Vec<&str> = lhs.split_whitespace().collect();

	let expr = match tokens.as_slice() {
		[x] => Expr::Direct(parse_source(x)),

		["NOT", x] => Expr::Not(parse_source(x)),

		[a, "AND", b] => Expr::And(parse_source(a), parse_source(b)),

		[a, "OR", b] => Expr::Or(parse_source(a), parse_source(b)),

		[a, "LSHIFT", n] => {
			let shift: u8 = n.parse().expect("invalid shift amount");
			Expr::LShift(parse_source(a), shift)
		}

		[a, "RSHIFT", n] => {
			let shift: u8 = n.parse().expect("invalid shift amount");
			Expr::RShift(parse_source(a), shift)
		}

		_ => panic!("Could not parse line: {line}"),
	};

	(target, expr)
}

fn parse_circuit(input: &str) -> Circuit {
	input
		.lines()
		.filter(|l| !l.trim().is_empty())
		.map(parse_line)
		.collect()
}

fn eval_source(src: &Source, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
	match src {
		Source::Const(v) => *v,
		Source::Wire(name) => eval_wire(name, circuit, cache),
	}
}

fn eval_wire(name: &str, circuit: &Circuit, cache: &mut HashMap<String, u16>) -> u16 {
	if let Some(&v) = cache.get(name) {
		return v;
	}

	let expr = circuit
		.get(name)
		.unwrap_or_else(|| panic!("Unknown wire: {name}"));

	let value: u16 = match expr {
		Expr::Direct(src) => eval_source(src, circuit, cache),
		Expr::Not(src) => !eval_source(src, circuit, cache),
		Expr::And(a, b) => eval_source(a, circuit, cache) & eval_source(b, circuit, cache),
		Expr::Or(a, b) => eval_source(a, circuit, cache) | eval_source(b, circuit, cache),
		Expr::LShift(src, n) => eval_source(src, circuit, cache) << n,
		Expr::RShift(src, n) => eval_source(src, circuit, cache) >> n,
	};

	cache.insert(name.to_string(), value);
	value
}

pub fn signal_on_a(input: &str) -> u16 {
	let circuit = parse_circuit(input);
	let mut cache = HashMap::new();
	eval_wire("a", &circuit, &mut cache)
}

fn part1(input: &str) -> u16 {
	let circuit = parse_circuit(input);
	let mut cache = HashMap::new();
	eval_wire("a", &circuit, &mut cache)
}

fn part2(input: &str) -> u16 {
	// get the initial signal on a.
	let circuit = parse_circuit(input);
	let mut cache = HashMap::new();
	let a1 = eval_wire("a", &circuit, &mut cache);

	// override b to that signal.
	let mut circuit2 = circuit.clone();
	circuit2.insert("b".to_string(), Expr::Direct(Source::Const(a1)));

	// reset wires.
	let mut cache2 = HashMap::new();
	eval_wire("a", &circuit2, &mut cache2)
}

fn main() {
	let input = include_str!("../input.txt");
	println!("Part 1: {}", part1(input));
	println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";

	#[test]
	fn example_circuit_values() {
		let circuit = parse_circuit(EXAMPLE);
		let mut cache = HashMap::new();

		assert_eq!(eval_wire("d", &circuit, &mut cache), 72);
		assert_eq!(eval_wire("e", &circuit, &mut cache), 507);
		assert_eq!(eval_wire("f", &circuit, &mut cache), 492);
		assert_eq!(eval_wire("g", &circuit, &mut cache), 114);
		assert_eq!(eval_wire("h", &circuit, &mut cache), 65412);
		assert_eq!(eval_wire("i", &circuit, &mut cache), 65079);
		assert_eq!(eval_wire("x", &circuit, &mut cache), 123);
		assert_eq!(eval_wire("y", &circuit, &mut cache), 456);
	}
}
