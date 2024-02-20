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
        nodelay(screen, false);
        // Creates new game and returns Game
        let mut game = Game::new(screen, last_config);
        // Limit fps
        let pause = 1000/(game.config.fps as u64);
        // A Round's loop for generating frames
        clear();
        nodelay(screen, true);
        while game.is_playing {
            // Pass pressed key to game to proccess what to do
            // move/reset/quit
            game.tick(Into::<char>::into(getch() as u8));
            // Render to terminal
            game.render();
            // Apply FPS limit
            thread::sleep(time::Duration::from_millis(pause));
        }
        // Show results and returns round score
        // TODO .. 
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
