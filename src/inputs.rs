// Inputs for both menu and game
pub enum Inputs{
    Up,
    Down,
    Left,
    Right,
    Esc,
    Wild
}

impl From<char> for Inputs {
    fn from(src: char) -> Inputs {
        match src{
            'w' => Inputs::Up,
            's' => Inputs::Down,
            'a' => Inputs::Left,
            'd' => Inputs::Right,
            'q' => Inputs::Esc,
            _   => Inputs::Wild
        }
    }
}