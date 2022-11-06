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
pub fn multiplier(a: u32, b: u32) -> u32
{
    let mut res = 0;
    for i in 0..32 {
        if (b & (1 << i)) != 0 {
            res = crate::adder::adder(res, a << i);
        }
    }
    return res;
}