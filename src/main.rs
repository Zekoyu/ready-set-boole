mod gray_code;
mod adder;
mod multiplier;
mod eval_formula;
mod print_truth_table;
mod negation_normal_form;

use eval_formula::eval_formula;

fn main() {

    let mut my_res: u32;
    let mut normal_res: u32;

    // println!("Looping from 0 to {}", std::u32::MAX);
    for i in 0..=1000 {

        for j in 10000..=11000 {
            my_res = adder::adder(i, j);
            normal_res = i + j;

            if my_res != normal_res {
                println!("Wrong result on {} + {} : expected {}, found {}", i, i, normal_res, my_res);
                std::process::exit(1);
            }

            my_res = multiplier::multiplier(i, j);
            normal_res = i * j;

            if my_res != normal_res {
                println!("Wrong result on {} * {} : expected {}, found {}", i, i, normal_res, my_res);
                std::process::exit(1);
            }

        }
    }
    println!("Adder / multiplier works fine");

    for i in 0..16 {
        println!("Gray code of {:02}: {:02}", i, gray_code::gray_code(i));
    }




    println!("{}", eval_formula("10&"));
    // false
    println!("{}", eval_formula("10|"));
    // true
    println!("{}", eval_formula("11>"));
    // true
    println!("{}", eval_formula("10="));
    // false
    println!("{}", eval_formula("1011||="));
    // true

    print_truth_table::print_truth_table("AB&C|");
    print_truth_table::print_truth_table("A!");

    println!("{}", negation_normal_form::negation_normal_form("AB&!"));

    std::process::exit(0);
}