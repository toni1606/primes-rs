use std::env::args;
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
fn start_threads(thread_count: u8, max_num: u32) -> Vec<Vec<u32>> {
	let numbers_per_thread = max_num / thread_count as u32;
	let test_limit = (max_num as f64).sqrt() as u32;

	let mut threads = Vec::with_capacity(thread_count as usize);
	for i in 0..thread_count {
		let min = 1 + (numbers_per_thread * i as u32);
		let max = min + numbers_per_thread;

		threads.push(thread::spawn( move || {
			let mut out: Vec<u32> = Vec::new();
			
			for i in min..max {
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

			out
		}));
	}

	let mut b: Vec<Vec<u32>> = Vec::new();

	for thread in threads {
		b.push(thread.join().unwrap());
	}

	b
}

fn main() {
	start_threads(8, parse_argument().expect("Invalid number entered"));
	println!("Hello, world!");
}
