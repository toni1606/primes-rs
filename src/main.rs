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

// TODO: try converting fors to iterators
fn start_threads(thread_count: u8, max_num: u32) {
	let numbers_per_thread = max_num / thread_count as u32;

	let sieve = |min: u32, max: u32, test_limit: u32| {
		let mut out: Vec<u32> = Vec::new();
		
		for i in min..max + 1 {
			let mut is_prime: bool = true;
			for j in 2..test_limit + 1 {
				if i % j == 0 {
					is_prime = false;
				}
			}

			if is_prime {
				out.push(i);
			}
		}

		// out
	};

	let v = Vec::with_capacity(thread_count as usize);
	let mut min = 1;
	for i in 0..thread_count {
		v.push(thread::spawn(sieve(min, min + numbers_per_thread, (max_num as f64).sqrt()  as u32 )));
	}
}

fn main() {
	start_threads(2, 15);
	println!("Hello, world!");
}
