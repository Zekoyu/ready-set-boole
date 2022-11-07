pub fn eval_formula(formula: &str) -> bool
{
	let mut stack: Vec<char> = std::vec::Vec::new();

	for c in formula.chars() {
		if c == '0' || c == '1' {
			stack.push(c);
		} else if c == '!' {
			if stack.len() < 1
			{
				println!("Bad formula (wrong stack len on NOT)");
				return false;
			}

			let val: bool = if stack.pop().unwrap() == '0' { false } else { true };
			stack.push(if val == false { '1' } else { '0' }); // negate
		} else {
			if stack.len() < 2 {
				println!("Bad formula {} (wrong stack len on operator)", formula);
				return false;
			}

			let b: bool = if stack.pop().unwrap() == '0' { false } else { true };
			let a: bool = if stack.pop().unwrap() == '0' { false } else { true };

			let res: bool = match c {
				'&' => a && b,
				'|' => a || b,
				'^' => a ^ b,
				'>' => !a || b, // https://fr.wikipedia.org/wiki/Proposition_contrapos%C3%A9e#Logique_classique : A => B  = !A || B
				'=' => a == b,
				_ => {
					println!("Unknown character {c}");
					return false;
				}
			};

			stack.push(if res == false { '0' } else { '1' });
		}
	}

	if stack.len() != 1 {
		println!("Bad formula (wrong final stack len)");
		return false;
	}

	if stack.pop().unwrap() == '1' { true } else { false }
}