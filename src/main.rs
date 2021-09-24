use std::env::args;
use std::num;
use std::thread;

fn parse_argument() -> Option<u32> {
	let args = args().collect::<Vec<String>>();
	
	if let Some(a) = args.get(1) {
		Some(a.parse::<u32>().expect("Invalid argument entered, insert number"))
	} else {
		None
	}
}

fn prime_sieve(max_num: u32) -> Vec<u32> {
	let mut out: Vec<u32> = Vec::new();
	for i in 1..max_num {
		let mut is_prime: bool = true;
		for  j in 2..((max_num as f64).sqrt() + 1.0) as u32 {
			if i % j == 0 {
				is_prime = false;
			}
		}

		if is_prime {
			out.push(i);
		}
	}

	out
}

fn main() {
	println!("Hello, world!");
	println!("primes: {:?}", prime_sieve(parse_argument().unwrap()));
}
