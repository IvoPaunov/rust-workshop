#[macro_use]
extern crate seed;
use std::cmp::Ordering;
use seed::dom_types::{El};
use seed::prelude::*;
use js_sys::Math::random;

// Model

struct Model {
	secret_number: u32,
	pub guess: String,
	pub won: bool,
	pub msg: String
}

impl Default for Model {
	fn default() -> Self {
		Self {
			secret_number: (random() * 100.0) as u32,
			msg: String::new(),
			guess: String::new(),
			won: false
		}
	}
}

// Update

#[derive(Clone)]
enum Msg {
	ChangeGuess(String)
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
	match msg {
		Msg::ChangeGuess (guess)=> {
			model.guess = guess.to_string();

			let parsed: u32 = match guess.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					model.msg = "Sir, please enter valid integer number!".to_string();
					return;
				}
			};
			
			model.msg = match parsed.cmp(&model.secret_number) {
				Ordering::Less => "Too small!".to_string(),
				Ordering::Greater => "Too big!".to_string(),
				Ordering::Equal => "You win!".to_string()
			};			
		},
	}
}

// View

fn view(model: &Model) -> El<Msg> {
	div![
		div![
			format!("Your guess: {}", model.guess)
		],
		div![
			format!("Info: {}", model.msg)
		],
		input![ attrs!{At::Value => model.guess}, input_ev(Ev::Input, Msg::ChangeGuess) ]
	]
}

#[wasm_bindgen]
pub fn render() {
	seed::App::build(Model::default(), update, view)
		.finish()
		.run();
}
