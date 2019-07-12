#[macro_use]
extern crate seed;
use js_sys::Math::random;
use seed::dom_types::El;
use seed::prelude::*;
use std::cmp::Ordering;

// Model

const MAX_ATTEMPTS: u32 = 3;

struct Model {
	secret_number: u32,
	pub attempts: u32,
	pub guess: String,
	pub game_active: bool,
	pub msg: String,
}

impl Default for Model {
	fn default() -> Self {
		Self {
			secret_number: (random() * 100.0) as u32,
			attempts: 0,
			msg: String::new(),
			guess: String::new(),
			game_active: true,
		}
	}
}

// Update

#[derive(Clone)]
enum Msg {
	ChangeGuess(String),
	SubmitGuess

}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
	match msg {
		Msg::SubmitGuess => {
			let parsed: u32 = match model.guess.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					model.msg = "Sir, please enter valid integer number!".to_string();
					return;
				}
			};
			model.msg = match parsed.cmp(&model.secret_number) {
				Ordering::Less => "Too small!".to_string(),
				Ordering::Greater => "Too big!".to_string(),
				Ordering::Equal => "You win!".to_string(),				
			};
		},
		Msg::ChangeGuess(guess) => {
			model.guess = guess;
		}
	}
}

// View

fn view(model: &Model) -> El<Msg> {
	div![
		div![format!("Your guess: {}", model.guess)],
		div![format!("Info: {}", model.msg)],
		input![
			attrs! {At::Value => model.guess},
			input_ev(Ev::Input, Msg::ChangeGuess)
		],
		button![
			simple_ev(Ev::Click, Msg::SubmitGuess),
			"Submit your guess"
		]
	]
}

#[wasm_bindgen]
pub fn render() {
	seed::App::build(Model::default(), update, view)
		.finish()
		.run();
}
