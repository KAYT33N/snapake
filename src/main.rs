#[allow(unused_imports)]
use std::{
    thread,
    time
};
use ncurses::{*};
mod game;
use game::{Game, Config};

fn main() {
    // Start ncurses.
    let screen = initscr();
    clear();
    refresh();
    noecho();
    // Main game Loop
    let mut best_score = 0;
    // holds last config in case of restarting game
    let mut last_config : Option<Config> = None;
    loop {
        // Creates new game and returns Game
        let mut game = Game::new(screen, last_config);
        best_score = best_score.max(game.start());
        // wait for user input
        nodelay(screen, false);
        if Into::<char>::into(getch() as u8) == 'q' {
            break;
        }
        last_config = Some(game.config.clone());
    }
    /* Terminate ncurses. */
    endwin();
}
