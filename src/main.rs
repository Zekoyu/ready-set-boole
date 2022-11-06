// https://en.wikipedia.org/wiki/Adder_(electronics)
fn adder(a: u32, b: u32) -> u32
{
    let mut res: u32 = 0;
    let mut carry: bool = false;
    for i in 0..32 {
        let mut add: u32 = (a & (1 << i)) ^ (b & (1 << i)); // output bit is a ^ b
        if add != 0 {
            add = 1; // set to 1 instead of power of 2
        }

        if add == 0 && carry {
            add = 1;
            carry = false; // useless
        }
        // if only 1 bit in a or b and carry, result is 0 and carry is 1 (eg. 0011 + 0110 = 1001 and not 1101)
        // see wikipedia truth table
        else if carry {
            add = 0;
        }

        // if we still have carry carry it to next bit
        if carry == false {
            carry = (a & (1 << i)) & (b & (1 << i)) != 0; // carry bit is set to 1 if a == 1 and b == 1 (a & b)
        }
        res = res | add << i; // set bit (add is target bit)
    }
    return res;
}

// https://en.wikipedia.org/wiki/Binary_multiplier
//       1011   (this is binary for decimal 11)
//     x 1110   (this is binary for decimal 14)
//     ======
//      0000   (this is 1011 x 0)
//     1011    (this is 1011 x 1, shifted one position to the left)
//    1011     (this is 1011 x 1, shifted two positions to the left)
// + 1011      (this is 1011 x 1, shifted three positions to the left)
// =========
//  10011010   (this is binary for decimal 154)

// 00011
// 00011
// 11 + 110
fn multiplier(a: u32, b: u32) -> u32
{
    let mut res = 0;
    for i in 0..32 {
        if (b & (1 << i)) != 0 {
            res = adder(res, a << i);
        }
    }
    return res;
}

// https://en.wikipedia.org/wiki/Gray_code#Converting_to_and_from_Gray_code
fn gray_code(n: u32) -> u32
{
    return n ^ (n >> 1);
}

fn main() {

    let mut my_res: u32;
    let mut normal_res: u32;

    // println!("Looping from 0 to {}", std::u32::MAX);
    for i in 0..=1000 {

        for j in 10000..=11000 {
            my_res = adder(i, j);
            normal_res = i + j;

            if my_res != normal_res {
                println!("Wrong result on {} + {} : expected {}, found {}", i, i, normal_res, my_res);
                std::process::exit(1);
            }

            my_res = multiplier(i, j);
            normal_res = i * j;

            if my_res != normal_res {
                println!("Wrong result on {} * {} : expected {}, found {}", i, i, normal_res, my_res);
                std::process::exit(1);
            }

        }
    }
    println!("Adder / multiplier works fine");

    for i in 0..16 {
        println!("Gray code of {:02}: {:02}", i, gray_code(i));
    }


    std::process::exit(0);
}