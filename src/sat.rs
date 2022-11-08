// same algo as truth table just return true if one result is true (with eval_formula)
pub fn sat(formula: &str) -> bool
{
	let mut operators: Vec<char> = std::vec::Vec::new();

	for c in formula.chars() {
		if c.is_alphabetic() && c.is_uppercase() && !operators.contains(&c) {
			operators.push(c);
		}
	}

	if operators.len() == 0 {
		return false;
	}

	for i in 0..1<<operators.len() {
		let mut operators_bool: Vec<bool> = std::vec::Vec::new();

		for j in (0..operators.len()).rev() {
			operators_bool.push((i & (1 << j)) != 0);
		}

		let mut formula_clone = String::from(formula);
		for j in 0..operators.len() {
			let replace = if operators_bool[j] == false { '0'.to_string() } else { '1'.to_string() };
			formula_clone = formula_clone.replace(operators[j], &replace);
		}

		if crate::eval_formula(formula_clone.as_str()) == true {
			return true;
		}
	}

	return false;
}