// https://en.wikipedia.org/wiki/Gray_code#Converting_to_and_from_Gray_code
pub fn gray_code(n: u32) -> u32
{
    return n ^ (n >> 1);
}