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
            tmp.init();
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

    // Adds player, enemies, foods to game
    fn init(&mut self) {
        // get rng
        let mut rng = rand::thread_rng();
        println!("making stones");
        // generate stones
        for i in 0..self.maxy{
            self.world.push(Vec::new());
            for j in 0..self.maxx{
                if 
                    rng.gen::<f64>() < self.config.stones_chance
                    || i == 0          || j == 0
                    || i > self.maxy-4 || j == self.maxx-1
                {
                    self.world[i].push(Types::Stone);
                }else{
                    self.world[i].push(Types::Empty);
                }
            }
        }
        // born player
        self.player = self.gen_rand_point();
        // spawn enemies
        while self.enemies.len() != self.config.enemies_count {
            let tmp_point = self.gen_rand_point();
            self.enemies.push(tmp_point);
        }
        // place foods
        while self.foods.len() != self.config.foods_count {
            self.foods.push(self.generate_food());
        }
    }

    fn generate_food(&self) -> Food {
        let mut rng = rand::thread_rng();
        let fps = self.config.fps;
        let foods_min_age = self.config.foods_min_age;
        let foods_max_age = self.config.foods_max_age;
        let foods_age_range_in_ticks = 
            (foods_min_age*fps)..=(foods_max_age*fps);
        let tmp_pos = self.gen_rand_point();
        let tmp_age = rng.gen_range(foods_age_range_in_ticks.clone()); 
        Food{
            pos: tmp_pos,
            age: tmp_age
        }
    }

    // Generates a random point until it finds an empty one
    fn gen_rand_point(&self) -> Point {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..self.maxx);
        let mut y = rng.gen_range(0..self.maxy);
        while 
            self.world[y][x] != Types::Empty
            || self
                .enemies
                .iter()
                .position(|item| item.x == x as i32 && item.y == y as i32)
                .is_some()
            || self
                .foods
                .iter()
                .position(|item| item.pos.x == x as i32 && item.pos.y == y as i32)
                .is_some()
            {
            x = rng.gen_range(0..self.maxx);
            y = rng.gen_range(0..self.maxy);
        }
        Point{
            x: x as i32,
            y: y as i32
        }
    }

    // games logic
    pub fn tick(&mut self, input: char){
        // React to input
        match input {
            'q' => {
                self.is_playing = false;
            },
            'w' => {
                self.move_player_if_possible(0, -1);
            },
            'a' => {
                self.move_player_if_possible(-1, 0);
            },
            's' => {
                self.move_player_if_possible(0, 1);
            },
            'd' => {
                self.move_player_if_possible(1, 0);
            },
            _ => {
                // Silence is gold
            }
        }
        // get rng
        let mut rng = rand::thread_rng();
        // Move enemies
        for enemy in &mut self.enemies {
            // move verticaly towards player
            if rng.gen::<f64>() < self.config.enemies_level {
                if enemy.y > self.player.y {
                    enemy.y -= 1;
                }
                if enemy.y < self.player.y {
                    enemy.y += 1;
                }
            }
            // move horizontaly towards player
            if rng.gen::<f64>() < self.config.enemies_level {
                if enemy.x > self.player.x {
                    enemy.x -= 1;
                }
                if enemy.x < self.player.x {
                    enemy.x += 1;
                }
            }
        }
        // Death check
        let is_dead = self
            .enemies.iter()
            .position(|enemy| enemy == &self.player)
            .is_some();
        if is_dead {
            self.tip = String::from("YOU ARE DEAD !");
            self.is_playing = false;
        }
        // Eat check
        let food_to_eat_index = self
            .foods.iter()
            .position(|food| food.pos == self.player);
        if food_to_eat_index.is_some() {
            self.foods[food_to_eat_index.unwrap()] 
                = self.generate_food();
            self.score += 10;
            self.tip = String::from("ate food + 10");
        }
        // Move foods if expired
        for i in 0..self.config.foods_count {
            self.foods[i].age -= 1;
            if self.foods[i].age < 5 {
                self.foods[i] = self.generate_food();
            }
        }
    }

    // Moves player if there isnt stones
    // Give delta move + 1 as arguments
    fn move_player_if_possible(&mut self, dx: i32, dy: i32) {
        let new_y = self.player.y + dy ;
        let new_x = self.player.x + dx ;
        if self.world[new_y as usize][new_x as usize] != Types::Stone {
            self.player.x = new_x;
            self.player.y = new_y;
        }
    }

    // Draw frame in terminal
    pub fn render(&self) {
        // Draw stones
        for i in 0..self.maxy {
            for j in 0..self.maxx {
                wmove(self.screen, i as i32, j as i32);
                addch(self.world[i][j].into());
            }
        }
        // Draw Tips
        wmove(self.screen, (self.maxy-2) as i32, 1);
        addstr(
            format!(
                "  {}  |  Score :  {}  ",
                self.tip,
                self.score
             ).as_str()
         );
        // Draw enemies
        for enemy in &self.enemies {
            wmove(
                self.screen, 
                enemy.y, 
                enemy.x
            );
            addch(Types::Enemy.into());
        }
        // Draw foods
        for food in &self.foods {
            wmove(
                self.screen, 
                food.pos.y, 
                food.pos.x
            );
            addch(Types::Food.into());
        }
        // Draw player
        wmove(
            self.screen, 
            self.player.y as i32, 
            self.player.x as i32
        );
        addch(Types::Player.into());
        // Render
        wmove(
            self.screen,
            (self.maxy-1) as i32,
            (self.maxx-1) as i32
        );
        refresh();
    }
}