use std::collections::BTreeSet;

pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32>
{
	let formula = crate::negation_normal_form::negation_normal_form(formula);

	let mut stack: Vec<BTreeSet<i32>> = vec![];
	let res: Vec<i32> = vec![];

	let mut sets_list: Vec<BTreeSet<i32>> = vec![];
	for i in 0..sets.len() {
		sets_list.push(BTreeSet::new());
		for j in 0..sets[i].len() {
			sets_list[i].insert(sets[i][j]);
		}
	}

	let mut global_set: BTreeSet<i32> = BTreeSet::new();
	for i in 0..sets_list.len() {
		let clone = global_set.clone();
		let union: Vec<&i32> = clone.union(&sets_list[i]).clone().collect();
		for j in 0..union.len() {
			let val: i32 = union[j].clone();
			global_set.insert(val);
		}
	}

	for c in formula.chars() {
		if c.is_alphabetic() && c.is_uppercase() {
			let set = sets[c as usize - 'A' as usize].iter().cloned().collect();
			stack.push(set);

		} else if c == '!' {
			if stack.len() < 1
			{
				println!("Bad formula (wrong stack len on NOT)");
				return res;
			}

			let mut set = stack.pop().unwrap();
			set = global_set.difference(&set).cloned().collect();
			stack.push(set);

		} else {
			if stack.len() < 2 {
				println!("Bad formula {} (wrong stack len on operator)", formula);
				return res;
			}

			let b = stack.pop().unwrap();
			let a = stack.pop().unwrap();

			match c {
				'&' => stack.push(a.intersection(&b).cloned().collect()),
				'|' =>  stack.push(a.union(&b).cloned().collect()),
				_ => {
					println!("Unknown character {c}");
					return res;
				}
			};
		}
	}

	return stack.pop().unwrap().into_iter().collect();
}