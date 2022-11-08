mod gray_code;
mod adder;
mod multiplier;
mod eval_formula;
mod print_truth_table;
mod negation_normal_form;
mod tester;
mod conjunctive_normal_form;
mod sat;
mod powerset;
mod eval_set;

use eval_formula::eval_formula;
use tester::*;

use crate::powerset::powerset;

fn main() {

    // let mut my_res: u32;
    // let mut normal_res: u32;

    // // println!("Looping from 0 to {}", std::u32::MAX);
    // for i in 0..=1000 {

    //     for j in 10000..=11000 {
    //         my_res = adder::adder(i, j);
    //         normal_res = i + j;

    //         if my_res != normal_res {
    //             println!("Wrong result on {} + {} : expected {}, found {}", i, i, normal_res, my_res);
    //             std::process::exit(1);
    //         }

    //         my_res = multiplier::multiplier(i, j);
    //         normal_res = i * j;

    //         if my_res != normal_res {
    //             println!("Wrong result on {} * {} : expected {}, found {}", i, i, normal_res, my_res);
    //             std::process::exit(1);
    //         }

    //     }
    // }
    // println!("Adder / multiplier works fine");

    // for i in 0..16 {
    //     println!("Gray code of {:02}: {:02}", i, gray_code::gray_code(i));
    // }




    // println!("{}", eval_formula("10&"));
    // // false
    // println!("{}", eval_formula("10|"));
    // // true
    // println!("{}", eval_formula("11>"));
    // // true
    // println!("{}", eval_formula("10="));
    // // false
    // println!("{}", eval_formula("1011||="));
    // // true

    // print_truth_table::print_truth_table("AB&C|");
    // print_truth_table::print_truth_table("A!");
    // // Xor equivalent in Negation Normal Form (A ^ B  =  A|B & (!A | !B))
    // print_truth_table::print_truth_table("AB|A!B!|&");
    // // AB|A!B!| == A|B  !A|!B

    // println!("---------- Negation normal form -----------");
    // println!("{}", negation_normal_form("AB&C|D|E&"));
    // println!("{}", negation_normal_form("AB>"));
    // println!("{}", negation_normal_form("AB|C&!"));
    // println!("{}", negation_normal_form("AB>"));
    // println!("{}", negation_normal_form("AIW>="));


    // println!("");
    // println!("");
    // println!("");

    // 0|0!&00!0!&&|0!&

    // ML=U>!  =  !((M == L) > U)
    // ML=U> = (M == L) > U  =  !((!M | L) && (!L | M)) | U ==  M!L|L!M|&!U|
    // ML=U>! = (M == L) > U  =  !(!((!M | L) && (!L | M)) | U) ==  M!L|L!M|&!U|!
    //                                                              M!L|L!M|&!U|!
    //                                                 my output =  M!L|&U!&

     /* M!L|L!M|&!U|!

                 NOT
                  /
                 OR
                /  \
               NOT  U
                /
               AND
              /   \
             OR   OR
           / \    / \
         NOT  L  NOT M
         /        /
         M        L


         First pass :       |          expected:
            AND             |
           /   \            |
         AND   NOT          |
         / \     /          |
        OR NULL  U          |
        /\                  |
      NOT L                 |
       /                    |
       M                    |

     */



    // let formula = "ML=U>!"; // M!L|&U!&
    // CN>CC^|
    //               without !       ML!&LM!&|U|
    // let nnf_formula = negation_normal_form(&formula);
    // let nnf_formula = "M!L|L!M|&!U|!";
    // println!("Formulas {} and {} are {}", formula, nnf_formula, if compare_formula(&formula, &nnf_formula) == true { "equivalent" } else { "not equivalent" } );

    for _ in 0..10 {
        let formula = generate_formula();
        let nnf_formula = negation_normal_form::negation_normal_form(&formula);
        println!("Formulas {} and {} are {}", formula, nnf_formula, if compare_formula(&formula, &nnf_formula) == true { "equivalent" } else { "not equivalent" } );
    }

    println!("CONJUNCTIVE FORMS");


    for _ in 0..10 {
        let formula = generate_formula();
        let cnf_formula = conjunctive_normal_form::conjunctive_normal_form(&formula);
        println!("Formulas {} and {} are {}", formula, cnf_formula, if compare_formula(&formula, &cnf_formula) == true { "equivalent" } else { "not equivalent" } );
    }

    println!("{}", sat::sat("AB|"));
    // true
    println!("{}", sat::sat("AB&"));
    // true
    println!("{}", sat::sat("AA!&"));
    // false
    println!("{}", sat::sat("AA^"));
    // false

    // see example with x, y, z : https://en.wikipedia.org/wiki/Power_set
    let powerpouet = powerset(&[1, 2, 3]);
    println!("Powerset of [1, 2, 3] : ");
    for i in 0..powerpouet.len() {
      print!("[");
      for j in 0..powerpouet[i].len() {
        print!("{}{}", powerpouet[i][j], if j < powerpouet[i].len() - 1 { ", " } else { "" });
      }
      println!("]");
    }

    let sets: Vec<Vec<i32>> = vec![
      vec![0, 1, 2],
      vec![0, 3, 4]
    ];
    let result = eval_set::eval_set("AB&", &sets);
    println!("Eval set:");
    print!("[");
    for i in 0..result.len() {
      print!("{}{}", result[i], if i < result.len() - 1 { ", " } else { "" });
    }
    println!("]");
    // [0]

    let sets = vec![
      vec![0, 1, 2],
      vec![3, 4, 5],
    ];
    let result = eval_set::eval_set("AB|", &sets);
    // [0, 1, 2, 3, 4, 5]
    print!("[");
    for i in 0..result.len() {
      print!("{}{}", result[i], if i < result.len() - 1 { ", " } else { "" });
    }
    println!("]");


    std::process::exit(0);
}