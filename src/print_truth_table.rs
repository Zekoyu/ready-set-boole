pub fn print_truth_table(formula: &str)
{
	let mut operators: Vec<char> = std::vec::Vec::new();

	for c in formula.chars() {
		if c.is_alphabetic() && c.is_uppercase() {
			operators.push(c);
		}
	}

	if operators.len() == 0 {
		return;
	}

	for _ in 0..operators.len() {
		print!("+---");
	}
	println!("+---+");

	for i in 0..operators.len() {
		print!("| {} ", operators[i]);
	}
	println!("| = |");

	for _ in 0..operators.len() {
		print!("|---");
	}
	println!("|---|");

	// There are 2^n solutions (n = operator count)
	for i in 0..1<<operators.len() {
		// println!("{i:03b} {}", operators.len()); // list all possible combinations
		let mut operators_bool: Vec<bool> = std::vec::Vec::new();
		// A = first bit from left to right, Z = last (if A is the first and Z last operator)
		// https://stackoverflow.com/questions/25170091/how-to-make-a-reverse-ordered-for-loop
		// print!("|");
		for j in (0..operators.len()).rev() {
			let op = (i & (1 << j)) != 0;
			operators_bool.push(op);
		}

		let mut formula_clone = String::from(formula);
		for j in 0..operators.len() {
			let replace = if operators_bool[j] == false { '0'.to_string() } else { '1'.to_string() };
			formula_clone = formula_clone.replace(operators[j], &replace);
			print!("| {} ", replace);
		}

		let res = crate::eval_formula(formula_clone.as_str());
		println!("| {} |", if res { '1' } else { '0' } );
	}
	for _ in 0..operators.len() {
		print!("+---");
	}
	println!("+---+");

}