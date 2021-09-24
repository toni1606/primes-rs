use std::env::args;

fn parse_argument() -> Option<u32> {
	let args = args().collect::<Vec<String>>();
	
	if let Some(a) = args.get(1) {
		Some(a.parse::<u32>().expect("Invalid argument entered, insert number"))
	} else {
		None
	}
}

fn main() {
	println!("Hello, world!");
	println!("arg: {:?}", parse_argument());
}
