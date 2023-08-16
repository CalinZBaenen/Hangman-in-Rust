use load::init_game;

use std::io::{stdout, stdin, Write, Read};






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



/// Enters the game.
pub(crate) fn enter_game(game:&mut Game) {
	let Some(state) = &mut game.state else {
		println!("The game failed to initialize properly. :(");
		game.active = false;
		return;
	};
	
	while state.chances > 0 && game.active {
		
		println!("\n{}", state.revealed_string);
		println!("{:?}", state.word.categories);
		if game.hints_enabled { println!("Hint: {}", state.word.hint); }
		println!("Guessed characters: {:?}; wrong-guesses left: {}", state.guessed, state.chances);
		print!("\nGuess a character: ");
		let _ = stdout().flush();
		
		if let Some(char) = stdin().bytes().next().and_then( |b| if let Ok(b) = b { Some(char::from(b)) } else { None } ) {
			let char = char.to_ascii_lowercase();
			
			if char.is_whitespace() { continue; }
			
			if state.guessed.contains(&char) {
				println!("\nYou already guessed '{char}'!");
				continue;
			}
			state.guessed.push(char);
			
			if state.word.name.contains(char) {
				if compute_new_reveal_string(state) == 0 {
					game.active = false;
					println!("You guessed the word correctly! :D");
				}
			} else {
				state.chances -= 1;
				println!("There is no '{char}' in this word!");
			}
		} else {
			println!("Could not read input! :(");
			game.active = false;
		}
		
	}
	if state.chances == 0 { println!("You guess incorrectly too many times! D:"); }
	
}



/// Prings the menu for the game.
pub(crate) fn menu(game:&mut Game) {
	let mut input = String::new();
	
	println!("Welcome to hangman- made in Rust.");
	while game.active {
		input.clear();
		
		print!(
			"What would you like To do? [P]lay  Toggle [H]ints ({})  [E]xit : ",
			if game.hints_enabled { "On" } else { "Off" }
		);
		let _ = stdout().flush();
		
		if let Ok(_) = stdin().read_line(&mut input) {
			input = input.to_ascii_lowercase().trim().to_string();
			if input.is_empty() { continue; }
			match input.as_str() {
				"play" | "p" => {
					init_game(game);
					enter_game(game);
				}
				"toggle hints" | "hints" | "h" => game.hints_enabled = !game.hints_enabled,
				"exit" | "e" => game.active = false,
				_ => {}
			}
		} else {
			println!("\nCould not read input! :(");
			panic!();
		}
	}
}