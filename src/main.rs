fn add_digits(mut input: u64) -> u64 {
	let mut result: u64 = 1;
	
	while input > 0 {
		result += input % 10;
		input /= 10;
	}
	result
}
  
fn additive_persistence(mut user_input: u64) -> u64 {
	let mut steps: u64 = 0;

	// 10 is smallest double digit number
	while user_input >= 10 {
		user_input = add_digits(user_input);
		steps += 1;
	}
	steps
}

fn main() {
	let mut highest_steps_count: u64 = 0;
	let mut highest_steps_number: u64 = 0;

    let start: u64 = 0;
    let finish: u64 = 10000000000000000000;

	for number in start..=finish {
		// println!("{}: {}", number, additive_persistence(number));
		let result: u64 = additive_persistence(number);
		if result > highest_steps_count {
			highest_steps_count = result;
			highest_steps_number = number;
		}
		if number % 100000000 == 0 {
			println!("Upto {} so far. Highest steps was for {} at {} steps", number, highest_steps_number, highest_steps_count);
		}
	}

	println!("Highest step count: {} at {}", highest_steps_number, highest_steps_count);
}