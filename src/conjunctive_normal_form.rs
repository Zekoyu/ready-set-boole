use crate::negation_normal_form::*;

/*

	A & (B | C) = (A & B) | (A & C)
	BC|A& = AB&AC&|

	A | (B & C) = (A | B) & (A | C)
	BC&A| = AB|AC|&
 */

pub fn conjunctive_normal_form(formula: &str) -> String
{
	let root = parse_formula_to_tree(formula);
	if root.is_none() {
		return String::new();
	}

	let mut root = root.unwrap();
	convert_tree_to_nnf(&mut root);

	convert_tree_from_nnf_to_cnf_rec(&mut root);

	return get_formula_postorder(root);
}


/*
	From

	  OR
	 /  \
	A   AND
	    / \
	   B   C

	To

  	  AND
     /   \
    OR    OR
    /\    /\
   A  B  A  C
 */
// Returns 1 if changed tree, 0 otherwise (used to count number of & to put at the end)
fn collapse_or_node_to_cnf(root: &mut Node) -> bool
{
	let left_node = root.left.as_mut().unwrap();
	let right_node = root.right.as_mut().unwrap();

	if left_node.value == '&' {
		let new_left = Some(Box::new(Node { left: root.right.clone(), right: left_node.left.clone(), value: '|' } ));
		let new_right = Some(Box::new(Node { left: root.right.clone(), right: left_node.right.clone(), value: '|' } ));
		root.value = '&';
		root.left = new_left;
		root.right = new_right;
		return true;
	} else if right_node.value == '&' {
		let new_left = Some(Box::new(Node { left: root.left.clone(), right: right_node.left.clone(), value: '|' } ));
		let new_right = Some(Box::new(Node { left: root.left.clone(), right: right_node.right.clone(), value: '|' } ));
		root.value = '&';
		root.left = new_left;
		root.right = new_right;
		return true;
	} else {
		return false;
	}
}

fn convert_tree_from_nnf_to_cnf_rec(root: &mut Node) -> bool
{
	let mut res = false;
	if root.left.is_some() {
		res |= convert_tree_from_nnf_to_cnf_rec(root.left.as_mut().unwrap());
	}

	if root.right.is_some() {
		res |= convert_tree_from_nnf_to_cnf_rec(root.right.as_mut().unwrap());
	}

	if root.value == '|' {
		res |= collapse_or_node_to_cnf(root);
	}

	return res;
}

/*
if !f {
	let mut cnt: u32 = 0;
	for c in formula.chars() {
		cnt += (c == '&') as u32;
	}
	let mut res = String::new();
	for c in formula.chars() {
		if c == '&' {
			continue;
		}
		res.push(c);
	}
	while cnt != 0 {
		res.push('&');
		cnt -= 1;
	}
	return res;
}

    } */