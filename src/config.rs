use ncurses::*;

#[derive(Clone, Copy)]
pub struct Config {
    pub fps         : usize,
    pub enemies_count: usize,
    pub foods_count  : usize,
    // lower means filler
    pub stones_chance: f64,
    // lower means easier
    pub enemies_level: f64,
    // in seconds
    pub foods_min_age: usize,
    pub foods_max_age: usize
}

impl Config{

    // Shows user graphical screen to change values
    pub fn get_config_interactive(
        last_config : Option<Config>, 
        screen      : ncurses::WINDOW,
        best_score  : usize,
        score       : Option<usize>
    ) -> Option<Config> {
        let defaults = Config {
            enemies_count: 3,
            foods_count  : 5,
            stones_chance: 0.03,
            enemies_level: 0.15,
            foods_min_age: 4,
            foods_max_age: 7,
            fps          : 30
        };
        let mut configs = last_config.unwrap_or(defaults);
        nodelay(screen, false);
        let mut focus = 0;
        loop{
            // print menu
            Self::print_configs_menu(
                screen, 
                focus, 
                configs, 
                best_score, 
                score
              );
            // Handle actions
            match Into::<char>::into(getch() as u8) {
                'w' => {
                    if focus > 1 {
                        focus -= 1;
                    }
                },
                's' => {
                    if focus < 8 {
                        focus += 1;
                    }
                },
                'a' => match focus {
                    1 => {configs.enemies_count -= 1},
                    2 => {configs.foods_count   -= 1},
                    3 => {configs.stones_chance -= 0.01},
                    4 => {configs.enemies_level -= 0.01},
                    5 => {configs.foods_min_age -= 1},
                    6 => {configs.foods_max_age -= 1},
                    7 => {configs.fps           -= 1},
                    8 => {break;},
                    _ => {},
                },
                'd' => match focus {
                    1 => {configs.enemies_count += 1},
                    2 => {configs.foods_count   += 1},
                    3 => {configs.stones_chance += 0.01},
                    4 => {configs.enemies_level += 0.01},
                    5 => {configs.foods_min_age += 1},
                    6 => {configs.foods_max_age += 1},
                    7 => {configs.fps           += 1},
                    8 => {break;},
                    _ => {},
                },
                'q' => {
                    return None;
                },
                _ => {
                    // Nothing to do
                }
            }
        }
        Some(configs)
}

fn print_configs_menu(
        screen      : ncurses::WINDOW, 
        focus       : i32, 
        configs     : Config,
        best_score  : usize,
        score       : Option<usize>
    ) {
        let base : i32 = 1;
        clear();
        let mut current = base;
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                "   /--------------SNAPAKE--------------\\ "
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                "  /---------Use `wasd` to move----------\\"
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " /----------Press `q`  to exit-----------\\"
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |>-------------------------------------<|"
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}number of enemies \t\t: {:03}  {}|",
                if focus == 1 {"-> "} else {"  "},
                configs.enemies_count,
                if focus == 1 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}number of foods \t\t: {:03}  {}|",
                if focus == 2 {"-> "} else {"  "},
                configs.foods_count,
                if focus == 2 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}stones density \t\t: {:0.02} {}|",
                if focus == 3 {"-> "} else {"  "},
                configs.stones_chance,
                if focus == 3 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}enemies intelligence \t: {:0.02} {}|",
                if focus == 4 {"-> "} else {"  "},
                configs.enemies_level,
                if focus == 4 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}foods minimum age in (s) \t: {:03}  {}|",
                if focus == 5 {"-> "} else {"  "},
                configs.foods_min_age,
                if focus == 5 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}foods maximum age in (s) \t: {:03}  {}|",
                if focus == 6 {"-> "} else {"  "},
                configs.foods_max_age,
                if focus == 6 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " |{}frames per second\t\t: {:03}  {}|",
                if focus == 7 {"-> "} else {"  "},
                configs.fps,
                if focus == 7 {"<-"} else {"  "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " | {0} Confirm {0} |",
                if focus == 8 {"~~~~~~~~~~~~~~"} else {"              "},
            ).as_str()
        );
        current += 1;
        wmove(screen, current, 0);
        addstr(
            format!(
                " \\-------+------------------------------/",
            ).as_str()
        );
        if score.is_some() {
            let score = score.unwrap();
            if score == best_score {
                current += 1;
                wmove(screen, current, 0);
                addstr(
                    format!(
                        "         | New HIGHSCORE : {}",
                        score
                    ).as_str()
                );
            }else{
                current += 1;
                wmove(screen, current, 0);
                addstr(
                    format!(
                        "           | Highscore : {}",
                        best_score
                    ).as_str()
                );
                current += 1;
                wmove(screen, current, 0);
                addstr(
                    format!(
                        "           | Your score : {}",
                        score
                    ).as_str()
                );

            }
        }
        // set curser to focused line
        wmove(screen, base+4+focus, 0);
        refresh();
    }
}