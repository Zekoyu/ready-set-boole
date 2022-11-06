// https://en.wikipedia.org/wiki/Adder_(electronics)
pub fn adder(a: u32, b: u32) -> u32
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