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
    let mut last_config : Option<Config> = None;
    loop {
        // Creates new game and returns Game
        let mut game = Game::new(screen, last_config);
        // runs game and returns result
        let score = game.start();
        // render results
        clear();
        if score > best_score {
            wmove(screen, 2, 0);
            addstr(format!("\tNew HIGHSCORE : {}", score).as_str());
            best_score = score;
        }else{
            wmove(screen, 2, 0);
            addstr(format!("\tBetter luck next time !").as_str());
            wmove(screen, 3, 0);
            addstr(format!("\thighscore  : {}", best_score).as_str());
            wmove(screen, 4, 0);
            addstr(format!("\tyour score : {}", score).as_str());
        }
        wmove(screen, 6, 0);
        addstr("\tPress `q` to exit");
        wmove(screen, 7, 0);
        addstr("\tOr press any other key to restart game");
        refresh();
        // Tries to prevent misclicks
        nodelay(screen, true);
        thread::sleep(time::Duration::from_millis(1000));
        let _ignore_this_input = getch();
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
