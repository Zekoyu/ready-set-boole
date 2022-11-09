pub fn map(x: u16, y: u16) -> f64
{
	let xf = x as f64;
	let yf: f64 = y as f64;
	let ret = xf / (1 << 16) as f64 + yf / ((1 as u64) << 32) as f64;
	return ret;
}

pub fn reverse_map(n: f64) -> (u16, u16)
{
	let mut y = (n * ((1 as i64) << 32) as f64) as u64;
    y &= ((1 as u64) << 16) - 1;
    let x = (n * ((1 as i64) << 16) as f64) as u64;
    (x as u16, y as u16)
}