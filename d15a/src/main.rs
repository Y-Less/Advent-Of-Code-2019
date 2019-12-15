mod intcode;
mod clear;
mod read;
use crate::intcode::Program;
use crate::intcode::ProgramState;
use crate::intcode::ProgramResult;
use std::str;
use crate::read::read;

type Pos = (usize, usize);

fn draw(grid: [[u8; 150]; 150], bot: Pos)
{
	crate::clear::clear();
	for (i, x) in grid.iter().enumerate()
	{
		if i == bot.1
		{
			let mut line = str::from_utf8(x).expect("").to_string();
			line.replace_range(bot.0 .. bot.0 + 1, "D");
			println!("{}", line);
		}
		else
		{
			let line = str::from_utf8(x).expect("");
			println!("{}", line);
		}
	}
}

fn run(prog: &mut ProgramState, dir: i64, grid: &mut [[u8; 150]; 150], bot: Pos) -> (bool, Pos)
{
	let mut bot = bot;
	const TILES: [u8; 4] = [' ' as u8, '#' as u8, '.' as u8, 'o' as u8];
	let mut res = if dir == 0 { ProgramResult::Request } else { ProgramResult::Response(dir) };
	loop
	{
		match prog.step(res)
		{
		ProgramResult::Response(v) =>
		{
			match v
			{
			0 =>
			{
				match dir
				{
				1 => grid[bot.1 - 1][bot.0] = TILES[1],
				2 => grid[bot.1 + 1][bot.0] = TILES[1],
				3 => grid[bot.1][bot.0 - 1] = TILES[1],
				4 => grid[bot.1][bot.0 + 1] = TILES[1],
				_ => {}
				}
			}
			1 =>
			{
				match dir
				{
				1 => bot.1 -= 1,
				2 => bot.1 += 1,
				3 => bot.0 -= 1,
				4 => bot.0 += 1,
				_ => {}
				}
				grid[bot.1][bot.0] = TILES[2];
			}
			2 =>
			{
				match dir
				{
				1 => bot.1 -= 1,
				2 => bot.1 += 1,
				3 => bot.0 -= 1,
				4 => bot.0 += 1,
				_ => {}
				}
				grid[bot.1][bot.0] = TILES[3];
				draw(*grid, bot);
				return (false, bot);
			}
			_ => {}
			}
		}
		ProgramResult::Request =>
		{
			draw(*grid, bot);
			return (true, bot);
		}
		ProgramResult::Halt =>
		{
			draw(*grid, bot);
			return (false, bot);
		}
		}
		res = ProgramResult::Request;
	}
}

fn main()
{
	let mut grid: [[u8; 150]; 150] = [[' ' as u8; 150]; 150];

	let mut prog = ProgramState::load("../inputs/d15.txt");
	
	let mut bot: Pos = (100, 100);
	run(&mut prog, 0, &mut grid, bot);

	loop
	{
		read();
		let dir = read();
		// < - 37
		// > - 39
		let dir =
			if dir == 37 { 3 }
			else if dir == 39 { 4 }
			else if dir == 38 { 1 }
			else if dir == 40 { 2 }
			else { continue; };
		let res = run(&mut prog, dir, &mut grid, bot);
		if !res.0
		{
			break;
		}
		bot = res.1;
	}
}
