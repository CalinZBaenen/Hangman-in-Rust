use game::{State, Game, Word};

use rand::{thread_rng, Rng};
use std::fs::read_to_string;





/// Loads the list of words from a special `words.txt` file.
pub(crate) fn load_wordlist() -> Result<Vec<Word>, ()> {
	#[derive(PartialEq, Clone, Copy, Eq)] enum ScanState {Hint, Name, Tags}
	
	let response = read_to_string("words.txt");
	
	if let Ok(content) = response {
		let mut words = Vec::new();
		
		let mut word_name = String::new();
		let mut word_tags = Vec::new();
		let mut tag_name  = String::new();
		let mut state     = ScanState::Name;
		let mut hint      = String::new();
		let mut col       = 0usize;
		
		for (idx, char) in content.chars().enumerate() {
			if (char == '\n' && col == 0) || idx == content.len()-1 {
				match state {
					ScanState::Hint => hint.push(char),
					ScanState::Name => word_name.push(char),
					ScanState::Tags => {
						tag_name.push(char);
						tag_name = tag_name.trim().to_string();
						if !tag_name.is_empty() { word_tags.push(tag_name.clone()); }
					}
				}
				
				state = ScanState::Name;
				
				if word_name.len() > 0 {
					words.push(Word {
						name: word_name.trim().to_string(),
						categories: word_tags.clone(),
						hint: hint.trim().to_string()
					});
					
					word_name.clear();
					word_tags.clear();
					hint.clear();
				}
			}
			if char == '\n' {
				col = 0;
				if state == ScanState::Name && !word_name.is_empty() { state = ScanState::Tags; }
				continue;
			}
			
			match state {
				ScanState::Name => {
					word_name.push(char.to_ascii_lowercase());
				}
				
				ScanState::Tags => {
					if char == ',' || char == ';' {
						tag_name = tag_name.trim().to_ascii_lowercase();
						if !tag_name.is_empty() {
							word_tags.push(tag_name.clone());
							tag_name.clear();
						}
						if char == ';' { state = ScanState::Hint; }
					} else { tag_name.push(char); }
				}
				
				ScanState::Hint => {
					if char == '\n' {
						state = ScanState::Name;
						continue;
					}
					hint.push(char);
				}
			}
			
			col += 1;
		}
		
		Ok(words)
	} else { Err(()) }
}



/// Loads the word-list and prepares the game-state.
pub(crate) fn init_game(game:&mut Game) {
	if let Ok(wordlist) = load_wordlist() {
		
		if wordlist.len() > 1 {
			let mut state = State::default();
			state.chances = 5;
			state.word = wordlist[ thread_rng().gen_range(0..wordlist.len()) ].clone();
			
			game.wordlist = wordlist;
			game.state = Some(state);
			
			return;
		} else { println!("(Can not play hangman without any words!)"); }
		
	}
	println!("Could not initialize word-list1! :(");
	game.active = false;
}