#[allow(unused_imports)]
use std::{
    thread,
    time
};
use ncurses::{*};
mod world;
use world::{Point, World};

fn main() {
    // Start ncurses.
    let screen = initscr();
    clear();
    refresh();
    noecho();
    cbreak();
    nodelay(screen, true);
    // Get max col & row number
    let mut maxx : i32 = 0;
    let mut maxy : i32 = 0;
    getmaxyx(screen, &mut maxy, &mut maxx);
    // Main game Loop
    loop {
        // Creates new world
        let world = World::new(
            // Pass maximum x & y
            Point{
                x: maxx as usize, 
                y: maxy as usize
            },
            // Number of foods
            5, 
            // Number of enemies
            3, 
            // Chance of turning a space into stone
            0.03
        );
        // Limit FPS
        let fps = 10;
        let pause = 1000/fps;
        // A Round's loop
        while world.is_playing {
            // Pass pressed key to world to proccess what to do
            // move/reset/quit
            world.tick(Into::<char>::into(getch() as u8));
            // Render to terminal
            world.render(screen);
            // Apply FPS limit
            thread::sleep(time::Duration::from_millis(pause));
        }
        // Show results
        // world.res();
        if Into::<char>::into(getch() as u8) == 'q' {
            break;
        }
    }
    /* Terminate ncurses. */
    endwin();
}
