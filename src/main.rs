use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn print_helper() {
	println!("'quit' or 'exit' to quit.");
}

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	// println!("The secret number is: {secret_number}");

	loop {

		println!("\n\nPlease input your guess.");

		let mut input: String = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		let input_u32: u32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				if input.trim() == "quit" || input.trim() == "exit" {
					break;
				} else if input.trim() == "help" {
					break;
				}

				print_helper();

				println!("Please input a number!");

				continue;
			},
		};

		println!("You guessed: {input_u32}");

		match input_u32.cmp(&secret_number) {
			Ordering::Less => {
				if secret_number - input_u32 < 10 {
					println!("Too small! But close!")
				} else {
					println!("Too small!")
				}
			},
			Ordering::Greater => {
				if input_u32 - secret_number < 10 {
					println!("Too big! But close!")
				} else {
					println!("Too big!")
				}
			},
			Ordering::Equal => {
				println!("You win!");
				break;
			},
		}
	}

}
