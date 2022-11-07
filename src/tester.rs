use crate::eval_formula;

use std::collections::BTreeMap;

pub fn generate_formula() -> String {
    let mut expr: Vec<String>  = vec![];

    for i in 'A'..='Z' {
        expr.push(i.to_string());
    }

    for _ in 0..15 {
        let a = &expr[rand::random::<usize>() % expr.len()];
        let b = &expr[rand::random::<usize>() % expr.len()];
        let c = match rand::random::<usize>() % 5 {
                0 => '&',
                1 => '|',
                2 => '^',
                3 => '>',
                4 => '=',
                _ => ' ',
        };
        let tmp = format!("{}{}{}{}", a, b, c, if rand::random::<usize>() % 5 == 0 { "!" } else { "" });
        expr.push(tmp);
    }

    expr.last().unwrap().clone()
}

pub fn eval_one(formula: &str, mp: &mut BTreeMap<char, bool>, mask: u32) -> bool{
    let mut i: u8 = 0;
    let len: u8 = mp.len() as u8;
    let mut t = String::from(formula);
    for (key, val) in mp.iter_mut() {
        *val = (mask >> (len - i - 1)) & 1 == 1;
        t = t.replace(*key, if *val { "1" } else { "0" });
        i += 1;
    }
    eval_formula(t.as_str())
}

pub fn compare_formula (formula1: &str, formula2: &str) -> bool {

    let mut mp: BTreeMap<char, bool> = BTreeMap::new();
    let mut mp2: BTreeMap<char, bool> = BTreeMap::new();

    for c in formula1.chars() {
        if c >= 'A' && c <= 'Z' {
            mp.insert(c, true);
        }
    }
    for c in formula2.chars() {
        if c >= 'A' && c <= 'Z' {
            mp2.insert(c, true);
        }
    }
    if mp != mp2 {
        return false;
    }

    for mask in 0..(1 << mp.len()) {
        if eval_one(formula1, &mut mp, mask) != eval_one(formula2, &mut mp, mask) {
            return false;
        }
    }
    true
}