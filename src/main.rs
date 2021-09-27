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

fn run(thread_count: u8, max_num: u32) -> Vec<u32> {
	let thread_handles = start_threads(thread_count, max_num / thread_count as u32);	

	let mut b: Vec<Vec<u32>> = Vec::new();

	for thread in thread_handles {
		b.push(thread.join().unwrap());
	}

	normalise_output(b)
}

fn start_threads(thread_count: u8, numbers_per_thread: u32) -> Vec<thread::JoinHandle<Vec<u32>>> {
	let mut thread_handles = Vec::with_capacity(thread_count as usize);
	for i in 0..thread_count {
		let min = 1 + (numbers_per_thread * i as u32);
		let max = min + numbers_per_thread;
		
		thread_handles.push(thread::spawn( move || {
			let mut out: Vec<u32> = Vec::new();
			
			for i in min..max {
				let test_limit = (i as f64).sqrt() as u32;
				let mut is_prime: bool = true;

				if i == 2 {	
					out.push(i);
					continue;
				} else if i % 2 == 0 {
					continue;
				}

				for j in 3..test_limit {
					if i % j == 0 {
						is_prime = false;
						break;
					}
				}

				if is_prime {
					out.push(i);
				}
			}

			out
		}));
	}

	thread_handles
}

fn normalise_output(initial_list: Vec<Vec<u32>>) -> Vec<u32> {
	let mut output_list: Vec<u32> = Vec::new();
	
	for list in initial_list {
		for element in list {
			output_list.push(element);
		}
	}

	output_list
}

fn main() {
	println!("{:?}", run(8, parse_argument().expect("Invalid number entered")));
}
