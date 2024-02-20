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
