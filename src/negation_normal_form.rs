#[derive(Clone)]
pub struct Node {
	// Option is for optional values (NULL equivalent)
	pub left: Option<Box<Node>>,
	pub right: Option<Box<Node>>,
	pub value: char,
}

impl Node {
	fn new(value: char) -> Node
	{
		return Node {
			left: None,
			right: None,
			value: value,
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



/*
	      AB|C&! after parsing

	     NOT
	    /
	   AND
	  /   \
	OR     C
	/\
   A  B


 collapse function
 becomes:
	   OR
	  /  \
    NOT  NOT
   /     /
  OR    C
  /\
 A  B


  becomes:
         OR
       /    \
     AND    NOT
     / \     /
   NOT NOT  C
   /   /
  A   B

 simpler form:
	 OR
	/  \
   AND C!
   / \
  A! B!

  to string (Post-order traversal (recursively left, right, lastly node))
  A!B!&C!|
 */
pub fn parse_formula_to_tree(formula: &str) -> Option<Node>
{
	let mut nodes: Vec<Node> = Vec::new();

	for c in formula.chars() {
		if c.is_alphabetic() && c.is_uppercase() {
			nodes.push(Node::new(c));
		} else if c == '!' {
			// Not node only has a left child, either another operator ( !(A&B) ) or a symbol ( !A )
			// In both case it's stored on the left child, so a NOT node ALWAYS have a left child
			let mut new_node = Node::new('!');

			let new_node_child={
				if nodes.len() >= 1 { nodes.pop().unwrap() }
				else {
					println!("Negation on nothing");
					return None;
				}
			};

			new_node.left = Some(Box::new(new_node_child));
			nodes.push(new_node);
		} else {
			let right_node: Node = {
				// wtf have i done ?? why 2 bro u stupid or what
				// if we have only one node, we want it on the left and not right
				// eg AB&C| AB& should be on the left and not the right and C on the right not the left
				if nodes.len() >= 2 {
					nodes.pop().unwrap()
				}
				else {
					println!("Wrong formula");
					return None;
				}
			};

			let left_node: Node = {
				if nodes.len() >= 1 {
					nodes.pop().unwrap()
				}
				else {
					println!("Wrong formula");
					return None;
				}
			};

				let mut node = Node::new(c);
				node.left = Some(Box::new(left_node));
				node.right = Some(Box::new(right_node));
				nodes.push(node);
		}
	}

	if nodes.len() != 1 {
		println!("Wrong final nodes length (invalid formula)");
		return None;
	}

	return nodes.pop();
}

pub fn negation_normal_form(formula: &str) -> String
{
	let root = parse_formula_to_tree(formula);

	if root.is_none() {
		return String::new();
	}
	// print_tree(&root, 0);
	// println!("");

	let mut root = root.unwrap();

	convert_tree_to_nnf(&mut root);
	return get_formula_postorder(root);
}


fn print_tree(root: &Node, level: u32)
{
	let level = level + 1;
	println!("");
	for _ in 0..level {
		print!("  ");
	}
	println!("-- Node{} {} --", level, root.value);
	for _ in 0..level {
		print!("  ");
	}
	print!("Node{} {} left :", level, root.value);
	if root.left.is_none() {
		println!("NULL");
	} else {
		print_tree(root.left.as_ref().unwrap(), level);
	}

	for _ in 0..level {
		print!("  ");
	}
	print!("Node{} {} right :", level, root.value);
	if root.right.is_none() {
		println!("NULL");
	} else {
		print_tree(root.right.as_ref().unwrap(), level);
	}
}


// Recursively collapses all ^, >, = and !  from leaves to root
// (with the exception of ! being the parent of a symbol and not expr)
fn collapse_not_node_to_nnf(mut root: &mut Node)
{
	if root.value.is_alphabetic() && root.value.is_uppercase() {
		return;
	}
	else if root.value == '!'{

		let left_node = root.left.as_ref().unwrap().clone();

		/*
		   NOT (root)
		   /
          NOT (left_node)
		  /
		  A (left_node.left.unwrap())
		 */
		// Check if double NOT (!!A => A)
		if left_node.value == '!' {
			let left_left_node = left_node.left.clone();
			// borrowing issues so set the root value to root->left->left->value and root->left to root->left->left->left
			root.value = left_left_node.unwrap().value;
			root.left = if left_node.left.is_some() { left_node.left.unwrap().left } else { None };
			if root.left.is_some() {
				collapse_not_node_to_nnf(root.left.as_mut().unwrap());
			}
			return;
		}

		let left_node = root.left.as_mut().unwrap();

		// if NOT on symbol, it's good, otherwise if NOT on expression, collapse expression
		if left_node.value.is_alphabetic() && left_node.value.is_uppercase() {
			return;
		} else {
			root.value = if left_node.value == '|' { '&' } else { '|' };
			// right node of the left node, which will become right node of root node
			let new_right_node = Box::new(Node { left: left_node.right.clone(), right: None, value: '!' });
			left_node.value = '!';
			root.right = Some(new_right_node);
			left_node.right = None;
			collapse_not_node_to_nnf(left_node);
		}
	}
}

pub fn get_formula_postorder(root: Node) -> String
{
	let mut str = String::from("");
	return get_tree_postorder_rec(root, &mut str).to_string();
}

fn get_tree_postorder_rec(root: Node, str: &mut String) -> &String
{
	if root.left.is_some() {
		get_tree_postorder_rec(*root.left.unwrap(), str);
	}

	if root.right.is_some() {
		get_tree_postorder_rec(*root.right.unwrap(), str);
	}

	str.push(root.value);
	return str;
}


/*
 A ^ B  =  A|B & (!A | !B)
 AB^ = AB|A!B!|&

    XOR                 AND
	/ \       =>       /   \
   A   B             OR     OR
                    /\       /\
				   A  B    NOT NOT
				           /   /
						   A   B


 A => B = (!A | B)
 AB> = A!B|

	CON           OR
	/ \     =>   /  \
   A   B       NOT   B
               /
			   A


 A = B  =  (A > B) & (B > A)  =  (!A | B) & (!B | A)
 AB= = AB>BA>& = A!B|B!A|&

   EQU                    AND
   / \      =>           /   \      => Then use the material condition algorithm
  A   B                CON  CON        on both left and right
                      / \    / \
					 A   B  B   A
 */
fn collapse_special_node_to_nnf(root: &mut Node)
{
	// first go to last level of nest
	match root.value {
		'^' | '>' | '=' => {
			let left_val = root.left.as_ref().unwrap().value;
			let right_val = root.right.as_ref().unwrap().value;

			if left_val == '>' || left_val == '=' || left_val == '^' {
				// recursive call to solve left first
				collapse_special_node_to_nnf(root.left.as_mut().unwrap());
			}

			if right_val == '>' || right_val == '=' || right_val == '^' {
				// recursive call to solve left first
				collapse_special_node_to_nnf(root.right.as_mut().unwrap());
			}
		},
		_ => { return; }
	};

	// at this point we are at the most deep level meaning there are no subtrees
	// which contains either ^, > or =, so apply our algorithm
	match root.value {
		'^' => {
			let left_save = root.left.as_ref().unwrap();
			let right_save = root.right.as_ref().unwrap();

			let new_left = Box::new(Node { left: Some(left_save.clone()), right: Some(right_save.clone()), value: '|' } );

			let not_node_left = Box::new(Node { left: Some(left_save.clone()), right: None, value: '!'} );
			let not_node_right = Box::new(Node { left: Some(right_save.clone()), right: None, value: '!'} );
			let new_right = Box::new(Node { left: Some(not_node_left), right: Some(not_node_right), value: '|'});
			root.value = '&';
			root.right = Some(new_right);
			root.left = Some(new_left);
		},
		'>' => {
			let new_left = Box::new(Node { left: Some(root.left.as_ref().unwrap().clone()), right: None, value: '!' } );
			root.value = '|';
			root.left = Some(new_left);
		},
		'=' => {
			let left_save = root.left.as_ref().unwrap();
			let right_save = root.right.as_ref().unwrap();

			let new_left = Box::new(Node { left: Some(left_save.clone()), right: Some(right_save.clone()), value: '>'});
			let new_right = Box::new(Node { left: Some(right_save.clone()), right: Some(left_save.clone()), value: '>'});
			root.value = '&';
			root.left = Some(new_left);
			root.right = Some(new_right);

			// Now solve the >
			collapse_special_node_to_nnf(root.left.as_mut().unwrap());
			collapse_special_node_to_nnf(root.right.as_mut().unwrap());
		},
		_ => { return; }
	};
}

// postorder otherwise it fails on some values
fn convert_tree_to_nnf_rec(root: &mut Node)
{
	// collapse NOT at last because if it finds a XOR or == or idk it won't handle it
	// so first convert all XOR, == etc. to their 'only & | !' equivalent

	if root.left.is_some() {
		convert_tree_to_nnf_rec(root.left.as_mut().unwrap());
	}

	if root.right.is_some() {
		convert_tree_to_nnf_rec(root.right.as_mut().unwrap());
	}

	if root.value == '!' {
		collapse_not_node_to_nnf(root)
	}
}

fn convert_tree_to_only_nnf_symbols_rec(root: &mut Node)
{
	if root.value == '^' || root.value == '>' || root.value == '=' {
		collapse_special_node_to_nnf(root);
	}
	// collapse NOT at last because if it finds a XOR or == or idk it won't handle it
	// so first convert all XOR, == etc. to their 'only & | !' equivalent

	if root.left.is_some() {
		convert_tree_to_only_nnf_symbols_rec(root.left.as_mut().unwrap());
	}

	if root.right.is_some() {
		convert_tree_to_only_nnf_symbols_rec(root.right.as_mut().unwrap());
	}
}

pub fn convert_tree_to_nnf(root: &mut Node)
{
	convert_tree_to_only_nnf_symbols_rec(root);
	convert_tree_to_nnf_rec(root);
}
