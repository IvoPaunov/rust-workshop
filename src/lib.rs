#[macro_use]
extern crate seed;
use js_sys::Math::random;
use seed::dom_types::El;
use seed::prelude::*;
use std::cmp::Ordering;

// Model

const MAX_ATTEMPTS: u32 = 8;

fn ralph() -> El<Msg> {
	div![
		id! {"ralph-wiggum"},
		div![
			class!["head"],
			div![class!["body", "head1"]],
			div![class!["no-border", "body", "head2"]],
			div![class!["hair", "hair-left", "hair1"]],
			div![class!["hair", "hair-left", "hair2"]],
			div![class!["hair", "hair-left", "hair3"]],
			div![class!["hair", "hair-left", "hair4"]],
			div![class!["hair", "hair-left", "hair5"]],
			div![class!["hair", "hair-left", "hair6"]],
			div![class!["hair", "hair-left", "hair7"]],
			div![class!["hair", "hair-right", "hair8"]],
			div![class!["hair", "hair-right", "hair9"]],
			div![class!["hair", "hair-right", "hair10"]],
			div![class!["hair", "hair-right", "hair11"]],
			div![class!["hair", "hair-right", "hair12"]],
			div![
				class!["body", "ear"],
				div![class!["inner1"]],
				div![class!["inner2"]],
				div![class!["no-border", "body", "clip"]]
			],
			div![class!["no-border", "body", "mouth5"]],
			div![class!["body", "mouth1"]],
			div![class!["no-border", "body", "mouth2"]],
			div![class!["no-border", "body", "mouth3"]],
			div![class!["no-border", "body", "mouth4"]],
			div![
				class!["left-eye", "eye"],
				div![class!["no-border", "pupil"]]
			],
			div![
				class!["right-eye", "eye"],
				div![class!["no-border", "pupil"]]
			],
			div![class!["body", "nose"]]
		]
	]
}

fn homer() -> El<Msg> {
	div![
		id! {"homer"},
		div![
			class!["head"],
			div![class!["hair1"]],
			div![class!["hair2"]],
			div![class!["body", "head-top"]],
			div![class!["no-border", "body", "head-main"]],
			div![class!["no-border", "m1"]],
			div![class!["no-border", "m2"]],
			div![class!["no-border", "m3"]],
			div![class!["no-border", "m4"]],
			div![class!["no-border", "neck1"]],
			div![class!["body", "neck2"]],
			div![
				class!["body", "ear"],
				div![class!["no-border", "inner1"]],
				div![class!["no-border", "inner2"]],
				div![class!["no-border", "body", "clip"]]
			],
			div![
				class!["mouth"],
				div![class!["mouth5"]],
				div![class!["mouth2"]],
				div![class!["mouth1"]],
				div![class!["mouth7"]],
				div![class!["no-border", "mouth3"]],
				div![class!["no-border", "mouth4"]],
				div![class!["no-border", "mouth6"]],
				div![class!["no-border", "mouth8"]]
			],
			div![
				class!["right-eye"],
				div![class!["no-border", "right-eye-pupil"]],
				div![class!["no-border", "body", "eyelid-top"]],
				div![class!["no-border", "body", "eyelid-bottom"]]
			],
			div![class!["body", "nose"]],
			div![class!["body", "nose-tip"]],
			div![
				class!["left-eye"],
				div![class!["no-border", "left-eye-pupil"]],
				div![class!["no-border", "body", "eyelid-top"]],
				div![class!["no-border", "body", "eyelid-bottom"]]
			]
		]
	]
}

#[derive(Default)]
struct Model {
	secret_number: u32,
	pub attempts: u32,
	pub guess: String,
	pub game_finished: bool,
	pub msg: String,
}

impl Model {
	fn randomize(mut self) -> Self {
		self.secret_number = (random() * 100.0) as u32;
		self
	}
}

// Update

#[derive(Clone)]
enum Msg {
	ChangeGuess(String),
	SubmitGuess,
	StartNewGame,
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
				Ordering::Equal => {
					model.game_finished = true;
					"Sir, you have won!".to_string()
				}
			};
			model.attempts += 1;

			if model.attempts == MAX_ATTEMPTS {
				model.game_finished = true;
				model.msg = "Sir, you loose!".to_string()
			}
		}
		Msg::ChangeGuess(guess) => {
			model.guess = guess;
		}
		Msg::StartNewGame => *model = Model::default().randomize(),
	}
}
// View

fn view(model: &Model) -> El<Msg> {
	div![
		class! {"game-board"},
		homer(),
		h2![format!("Your guess: {}", model.guess)],
		h2![format!("Info: {}", model.msg)],
		h2![format!("Attempts left: {}", MAX_ATTEMPTS - model.attempts)],
		if !model.game_finished {
			vec![
				input![
					attrs! {At::Value => model.guess},
					input_ev(Ev::Input, Msg::ChangeGuess)
				],
				button![
					simple_ev(Ev::Click, Msg::SubmitGuess),
					"Submit your guess",
					class! {"button", "inverse"}
				],
			]
		} else {
			vec![button![
				simple_ev(Ev::Click, Msg::StartNewGame),
				"Start new game"
			]]
		},
		ralph()
	]
}

#[wasm_bindgen]
pub fn render() {
	seed::App::build(Model::default().randomize(), update, view)
		.finish()
		.run();
}
