#[derive(PartialEq, Debug)]
pub enum Victory {
    GameOver,
    Won,
    Quit,
    Save(String),
    Load(String),
    None,
}
