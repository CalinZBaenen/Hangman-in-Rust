extern crate rand;

mod game;
mod load;

use game::*;





fn main() {
	let mut game = Game::default();
	
	menu(&mut game);
}