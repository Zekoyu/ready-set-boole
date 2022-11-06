// https://levelup.gitconnected.com/rust-binary-tree-30efdd355b60#:~:text=Today%20I'd%20like%20to,a%20Left%20and%20a%20Right.
enum Type
{
	Symbol,               // A..Z
	Negation,             // ! (NOT)
	Conjunction,          // & (AND)
	Disjunction,          // | (OR)
	ExclusiveDisjunction, // ^ (XOR)
	MaterialCondition,    // > (!A || B)
	Equivalence           // = (==)
}

// use std::boxed;

struct Node {
	// Option is for optional values (NULL equivalent)
	left: Option<Box<Node>>,
	right: Option<Box<Node>>,
	symbol_type: Type,
	value: char,
}

struct FormulaTree {
	root: Option<Node>,
}

impl FormulaTree
{
	fn new() -> FormulaTree
	{
		return FormulaTree {
			root: None
		}
	}
}

/*
!(A & B) == (!A | !B)
	AB&! :

	          First:

	AND         |         OR
	/ \         |         /\
   A   B        |        A  B

          Then (on NOT !):

	OR          |         AND
	/\          |         / \
  !A  !B        |       !A   !B


  We can implement this using only a stack as before for operations, and use the tree only when we encounters a NOT
  Then we just print as usual from top to bottom of stack
 */

 // !((A || B) && C) == (!(A || B) || !C) == ((!A && !B) || !C)
 // Negate all childs recursively until child is a symbol and not an expression
pub fn negation_normal_form(formula: &str) -> String
{
	let mut stack: Vec<char> = Vec::new();

	for c in formula.chars() {
		if (c.is_alphabetic() && c.is_uppercase()) {
			stack.push(c);
		} else {

		}
	}

	return std::string::String::from(formula);
}