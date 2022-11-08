// A powerset is just a set of all subsets of a given set (eg powerset of [x, y] is [ [], [x], [y], [x, y] ])
// https://en.wikipedia.org/wiki/Power_set
pub fn powerset(set: &[i32]) -> Vec<Vec<i32>>
{
	// vec![] is just for initializing vectors
	let mut res: Vec<Vec<i32>> = vec![];

	// 0 to 2^n (so complexity is O(2^n + n) which is simplified to O(2^n))
	// we can think of numbers as bit in a number (so set[0] is last bit, set[1] is the penultimate bit etc.)
	// Then just iterate over all possibilities (0 .. 2^n, which sets bits of number we want (eg 001 => only set[0], 011 => set[0] + set[1]))
	for i in 0..(1 << set.len()) {
		res.push(vec![]);
		for j in 0..set.len() {
			if i & (1 << j) != 0 {
				res[i].push(set[j]);
			}
		}
	}

	return res;
}