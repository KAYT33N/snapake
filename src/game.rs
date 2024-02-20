use rand::prelude::*;
use ncurses::{
    getmaxyx,
    addstr,
    addch,
    wmove,
    clear,
    refresh
};

#[derive(Clone, Copy)]
pub struct Config {
    pub fps      : usize,

    enemies_count: usize,
    foods_count  : usize,
    // lower means filler
    stones_chance: f64,
    // lower means easier
    enemies_level: f64,
    // in seconds
    foods_min_age: usize,
    foods_max_age: usize
}

// This is used to store player, enemies, foods Location
#[derive(Debug, PartialEq)]
struct Point{
    pub x: i32,
    pub y: i32
}

// Foods have location and remaining age
struct Food{
    pos: Point,
    age: usize
}

// Used to draw frames
#[derive(Clone,Copy,PartialEq)]
enum Types {
    Player,
    Empty,
    Enemy,
    Food,
    Stone
}
impl Into<u32> for Types {
    fn into(self) -> u32 {
        match self {
            Self::Player => 'X' as u32,
            Self::Empty  => ' ' as u32,
            Self::Enemy  => 'E' as u32,
            Self::Food   => 'F' as u32,
            Self::Stone  => '.' as u32,
        }
    }
}

// Main game's struct
pub struct Game {
    pub is_playing: bool,
    pub config    : Config,

    screen  : ncurses::WINDOW,
    maxx    : usize,
    maxy    : usize,
    player  : Point,
    foods   : Vec<Food>,
    world   : Vec<Vec<Types>>,
    enemies : Vec<Point>,
    score   : usize,
    tip     : String
}

impl Game{
    // creates 
    pub fn new(
        screen      : ncurses::WINDOW, 
        last_config : Option<Config>
      ) -> Self {
            // get max x & y
            let mut maxx : i32 = 0;
            let mut maxy : i32 = 0;
            getmaxyx(screen, &mut maxy, &mut maxx);
            // create empty game
            let mut tmp = Self{
                is_playing  : true,
                config      : Self::get_config(last_config),
                screen      : screen,
                maxx        : maxx as usize,
                maxy        : maxy as usize,
                player      : Point{x:0, y:0},
                foods       : Vec::new(),
                world       : Vec::new(),
                enemies     : Vec::new(),
                score       : 0,
                tip         : String::from("Press `q` to exit  |  use 'wasd' to move")
            };
            // fill it
            // tmp.init();
            // return game
            tmp
    }

    // Shows user graphical screen to change values
    fn get_config(last_config: Option<Config>) -> Config {
        Config {
            enemies_count: 3,
            foods_count  : 5,
            stones_chance: 0.03,
            enemies_level: 0.15,
            foods_min_age: 4,
            foods_max_age: 7,
            fps          : 30
        }
    }

}