use std::collections::HashMap;

const PHASES: usize = 100;
const DUP: usize = 10000;
const BASE: usize = 650;
const LEN: usize = BASE * DUP;

const ORIGINAL: [i32; BASE] = [5, 9, 7, 7, 6, 0, 3, 4, 0, 9, 5, 8, 1, 1, 6, 4, 4, 5, 4, 5, 3, 6, 7, 7, 9, 3, 1, 7, 9, 9, 8, 9, 6, 0, 2, 1, 4, 0, 9, 4, 8, 7, 1, 4, 4, 0, 6, 2, 3, 4, 6, 9, 4, 9, 7, 2, 8, 9, 4, 4, 8, 5, 0, 6, 6, 5, 2, 3, 5, 2, 5, 7, 4, 2, 5, 0, 3, 9, 8, 6, 7, 7, 1, 9, 1, 2, 0, 1, 9, 0, 3, 2, 9, 2, 2, 7, 8, 8, 4, 9, 4, 9, 0, 0, 6, 5, 5, 8, 5, 5, 4, 5, 8, 0, 8, 6, 9, 7, 9, 7, 6, 4, 6, 1, 7, 3, 7, 5, 5, 8, 0, 8, 0, 2, 5, 5, 8, 9, 6, 3, 5, 8, 7, 0, 2, 5, 7, 8, 4, 9, 1, 8, 8, 8, 2, 2, 1, 9, 6, 1, 0, 8, 3, 1, 9, 4, 0, 9, 9, 2, 3, 9, 9, 2, 0, 1, 7, 8, 2, 3, 8, 5, 6, 7, 4, 2, 2, 3, 2, 8, 4, 4, 1, 1, 4, 9, 9, 2, 3, 7, 6, 1, 9, 8, 0, 0, 1, 9, 3, 8, 7, 9, 7, 6, 8, 6, 6, 8, 2, 1, 0, 1, 6, 2, 1, 7, 6, 3, 9, 4, 6, 0, 7, 5, 0, 2, 2, 1, 8, 6, 0, 2, 6, 3, 3, 1, 5, 3, 7, 7, 2, 0, 6, 2, 9, 7, 3, 1, 4, 9, 5, 3, 3, 6, 5, 0, 5, 6, 2, 5, 5, 4, 9, 4, 2, 5, 7, 4, 5, 9, 3, 8, 7, 8, 0, 7, 3, 2, 3, 8, 2, 3, 2, 5, 6, 3, 6, 4, 9, 6, 7, 3, 8, 5, 8, 1, 6, 7, 6, 3, 5, 3, 7, 8, 6, 9, 5, 1, 9, 0, 3, 5, 6, 1, 5, 9, 7, 9, 6, 3, 4, 2, 2, 0, 4, 7, 5, 9, 3, 9, 3, 1, 5, 6, 2, 9, 4, 6, 5, 8, 3, 6, 6, 2, 7, 9, 9, 2, 2, 7, 3, 4, 2, 1, 3, 3, 8, 5, 1, 4, 4, 8, 9, 5, 1, 1, 6, 6, 4, 9, 7, 6, 8, 1, 8, 5, 9, 6, 6, 8, 6, 6, 2, 0, 2, 4, 1, 3, 3, 1, 4, 9, 3, 9, 6, 9, 2, 1, 7, 4, 2, 2, 3, 2, 1, 0, 4, 8, 4, 9, 3, 3, 6, 7, 8, 8, 6, 6, 4, 7, 8, 9, 4, 4, 1, 0, 4, 9, 7, 8, 8, 9, 0, 0, 1, 9, 7, 2, 8, 5, 6, 2, 0, 0, 1, 4, 1, 7, 7, 4, 6, 6, 5, 6, 6, 9, 9, 2, 8, 1, 9, 9, 2, 0, 2, 8, 3, 5, 6, 0, 0, 4, 8, 8, 8, 8, 6, 0, 1, 0, 3, 8, 0, 5, 4, 7, 2, 8, 6, 6, 6, 1, 5, 2, 4, 3, 5, 4, 4, 7, 8, 1, 3, 7, 7, 7, 4, 8, 6, 5, 4, 4, 7, 1, 7, 5, 0, 5, 6, 0, 8, 3, 0, 0, 9, 9, 0, 4, 8, 7, 4, 7, 5, 7, 0, 9, 2, 5, 9, 0, 2, 5, 7, 5, 7, 6, 5, 0, 5, 4, 8, 9, 8, 8, 9, 9, 5, 1, 2, 3, 0, 3, 9, 1, 7, 1, 5, 9, 1, 3, 8, 0, 9, 7, 3, 7, 5, 3, 3, 8, 4, 4, 4, 6, 1, 0, 8, 0, 9, 8, 9, 1, 6, 6, 7, 0, 9, 4, 0, 5, 1, 1, 0, 8, 3, 5, 9, 1, 3, 4, 0, 1, 7, 1, 2, 8, 0, 2, 8, 1, 7, 4, 2, 3, 0, 7, 2, 0, 3, 9, 8, 9, 6, 5, 9, 6, 0, 7, 1, 2];

//const PHASES: usize = 100;
//const DUP: usize = 1;
//const BASE: usize = 32;
//const LEN: usize = BASE * DUP;
//
//const ORIGINAL: [i32; BASE] = [6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4, 3, 1, 9, 8, 7, 3];
// 
//const PHASES: usize = 4;
//const DUP: usize = 1;
//const BASE: usize = 8;
//const LEN: usize = BASE * DUP;
//
//const ORIGINAL: [i32; BASE] = [1, 2, 3, 4, 5, 6, 7, 8];

fn gcd(a: usize, b: usize) -> usize
{
	let (mut a, mut b) = if a > b
	{
		(a, b)
	}
	else
	{
		(b, a)
	};

	while b != 0
	{
		let r = a % b;
		a = b;
		b = r;
	}

	a
}

fn lcd(a: usize, b: usize) -> usize
{
	a * b / gcd(a, b)
}

fn find_digit(idx: usize, phase: usize, cache: &mut HashMap<(usize, usize), i32>) -> i32
{
	if phase == 0
	{
		return ORIGINAL[idx % BASE];
	}
	let phase = phase - 1;
	let name = (phase, idx);
	match cache.get(&name)
	{
	Some(n) => return *n,
	None => {}
	}
	let len = if phase == 0 { lcd(BASE, (idx + 1) * 4) } else { LEN };
	let len = if len > LEN { LEN } else { len };
	let frac = (LEN / len) as i32;
	let mut total = 0;
	let mut j = idx;
	let mut e = j + idx + 1;
	let mut mul = 1;
	loop
	{
		if j >= e
		{
			j = e + idx + 1;
			e = j + idx + 1;
			mul = -mul;
		}
		if j >= len
		{
			break;
		}
		total += find_digit(j, phase, cache) * mul;
		j += 1;
	}
	let ret = total.abs() * frac % 10;
	cache.insert(name, ret);
	return ret;
}

fn main()
{
	let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
	println!("{}{}{}{}{}{}{}{}", find_digit(0, PHASES, &mut cache), find_digit(1, PHASES, &mut cache), find_digit(2, PHASES, &mut cache), find_digit(3, PHASES, &mut cache), find_digit(4, PHASES, &mut cache), find_digit(5, PHASES, &mut cache), find_digit(6, PHASES, &mut cache), find_digit(7, PHASES, &mut cache));
}

