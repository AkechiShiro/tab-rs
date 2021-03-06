#[derive(Debug, Clone)]
pub enum FuzzyEvent {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Insert(char),
    Delete,
    Enter,
    Resize(u16, u16),
}

#[derive(Debug, Clone)]
pub struct FuzzySelection(pub String);

#[derive(Debug, Clone)]
pub struct FuzzyShutdown;
