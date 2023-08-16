use std::io::{stdout, stdin, Write};






#[derive(Default, Clone, Debug)]
pub(crate) struct State {
	pub(crate) revealed_string:String,
	pub(crate) chances:u8,
	pub(crate) guessed:Vec<char>,
	pub(crate) word:Word
}



#[derive(Clone, Debug)]
pub(crate) struct Game {
	pub(crate) hints_enabled:bool,
	pub(crate) wordlist:Vec<Word>,
	pub(crate) active:bool,
	pub(crate) state:Option<State>
}

impl Game {
	fn start(&mut self) {
		let Some(state) = &mut self.state else {
			println!("The game failed to initialize properly. :(");
			self.active = false;
			return;
		};
		compute_new_reveal_string(state);
		
		let mut input = String::new();
		while state.chances > 0 && self.active {
			input.clear();
			
			println!("\n{}", state.revealed_string);
			println!("{:?}", state.word.categories);
			if self.hints_enabled { println!("Hint: {}", state.word.hint); }
			println!("Guessed characters: {:?}; wrong-guesses left: {}", state.guessed, state.chances);
			print!("\nGuess a character: ");
			let _ = stdout().flush();
			
			if stdin().read_line(&mut input).is_err() {
				println!("Could not read input! :(");
				self.active = false;
				return;
			}
			input = input.trim().to_ascii_lowercase();
			if input.len() > 1 {
				println!("You can not guess more than one character at a time!");
				continue;
			}
			let Some(char) = input.chars().nth(0) else { continue; };
			
			if state.guessed.contains(&char) {
				println!("\nYou already guessed '{char}'!");
				continue;
			}
			state.guessed.push(char);
			
			if !state.word.name.contains(char) {
				println!("There is no '{char}' in this word!");
				state.chances -= 1;
				continue;
			}
			if compute_new_reveal_string(state) == 0 {
				println!("You guessed the word correctly! :D");
				break;
			}
		}
	}
	
	
	
	/// Handles the entry menu for the game.
	pub(crate) fn enter(&mut self) {
		let mut input = String::new();
		
		println!("Welcome to hangman- made in Rust.");
		while self.active {
			input.clear();
			
			print!(
				"What would you like To do? [P]lay  Toggle [H]ints ({})  [E]xit : ",
				if self.hints_enabled { "On" } else { "Off" }
			);
			let _ = stdout().flush();
			
			if let Ok(_) = stdin().read_line(&mut input) {
				input = input.to_ascii_lowercase().trim().to_string();
				if input.is_empty() { continue; }
				match input.as_str() {
					"play" | "p" => {
						self.initialize();
						self.start();
					}
					"toggle hints" | "hints" | "h" => self.hints_enabled = !self.hints_enabled,
					"exit" | "e" => self.active = false,
					_ => {}
				}
			} else {
				println!("\nCould not read input! :(");
				self.active = false;
			}
		}
	}
}

impl Default for Game {
	fn default() -> Self {
		Game {
			hints_enabled:true,
			wordlist:Vec::new(),
			active:true,
			state:None
		}
	}
}



#[derive(Default, Clone, Debug)]
pub(crate) struct Word {
	pub(crate) categories:Vec<String>,
	pub(crate) hint:String,
	pub(crate) name:String
}





/// Computes the string to be revealed.
pub(crate) fn compute_new_reveal_string(state:&mut State) -> usize {
	state.revealed_string.clear();
	
	let mut hidden_chars = 0;
	for char in state.word.name.chars() {
		if !char.is_whitespace() {
			state.revealed_string.push(if state.guessed.contains(&char) { char } else { hidden_chars += 1; '_' });
		}
		else { state.revealed_string.push(' '); }
	}
	hidden_chars
}