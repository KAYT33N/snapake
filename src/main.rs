#[allow(unused_imports)]
use std::{
    thread,
    time
};
use ncurses::{*};
mod game;
use game::Game;
mod config;
use config::Config;

fn main() {
    // Start ncurses.
    let screen = initscr();
    clear();   
    refresh();
    noecho();
    // Main game Loop
    let mut best_score = 0;
    // holds last config in case of restarting game
    let mut config 
        : Option<Config> 
        = Config::get_config_interactive(
            None,
            screen,
            best_score,
            None
        );
    while config.is_some() {
        // Creates new game and returns Game
        let mut game = Game::new(screen, config.unwrap());
        // runs game and returns result
        let score  = game.start();
        best_score = best_score.max(score);
        // this prevents user from confusion after death
        thread::sleep(time::Duration::from_millis(1500));
        // renders results and returns configs
        config = Config::get_config_interactive(
                Some(game.config),
                screen,
                best_score,
                Some(score)
            );
    }
    /* Terminate ncurses. */
    endwin();
}
