use std::io;

fn main() -> io::Result<()> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer)?;

	let v: Vec<_> = buffer.split_whitespace().collect();

	let a : i128 = v.get(0).unwrap().parse().unwrap();
	let b : i128 = v.get(1).unwrap().parse().unwrap();

	println!("{}", a + b);
	Ok(())
}
