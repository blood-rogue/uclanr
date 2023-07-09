use rand::Rng;
use std::env;

mod words;

fn main() {
	let amount = get_amount();
	for _ in 0..amount {
		println!("{}", get_random_word());
	}
}

fn get_random_word() -> String {
	let mut rng = rand::thread_rng();
	let random_number = rng.gen_range(0..words::WORDS.len());
	words::WORDS[random_number].to_owned()
}

fn get_amount() -> u32 {
	env::args()
		.nth(1)
		.map(|amount| amount.trim().parse().unwrap_or(1))
		.unwrap_or(1)
}
