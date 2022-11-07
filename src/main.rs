mod gray_code;
mod adder;
mod multiplier;
mod eval_formula;
mod print_truth_table;
mod negation_normal_form;
mod tester;

use eval_formula::eval_formula;
use tester::*;

use crate::negation_normal_form::negation_normal_form;

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
        let nnf_formula = negation_normal_form(&formula);
        println!("Formulas {} and {} are {}", formula, nnf_formula, if compare_formula(&formula, &nnf_formula) == true { "equivalent" } else { "not equivalent" } );
    }
    // AB&A!B!&|

    std::process::exit(0);
}