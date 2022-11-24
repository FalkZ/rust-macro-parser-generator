use super::tokens::RawToken;

#[derive(Default, Clone, Debug)]
pub struct Position {
    pub column: u32,
    pub line: u32,
}
pub trait GetPosition {
    fn position(&self) -> Position;
}

pub trait TryGetPosition {
    fn try_position(&self) -> Option<Position>;
}

impl<T: TryGetPosition> TryGetPosition for Vec<T> {
    fn try_position(&self) -> Option<Position> {
        self.get(0).map(|v| v.try_position()).flatten()
    }
}

impl<T: RawToken> GetPosition for T {
    fn position(&self) -> Position {
        self.raw_token().position
    }
}

impl<T: GetPosition> TryGetPosition for T {
    fn try_position(&self) -> Option<Position> {
        Some(self.position())
    }
}
