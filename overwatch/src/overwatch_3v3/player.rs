use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, IntoEnumIterator)]
pub enum Player {
    One,
    Two,
    Three,
}

impl Default for Player {
    fn default() -> Player {
        Player::One
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct NumericPlayer(Player);

impl fmt::Debug for NumericPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.numeric())
    }
}

impl fmt::Display for NumericPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player{}", self.numeric())
    }
}

impl Player {
    #[inline]
    pub fn iter() -> PlayerEnumIterator {
        Player::into_enum_iter()
    }
    #[inline]
    pub fn numeric(self) -> usize {
        match self {
            Player::One => 1,
            Player::Two => 2,
            Player::Three => 3,
        }
    }

    pub fn cycle_next(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::Three,
            Player::Three => Player::One,
        }
    }

    #[inline]
    pub fn numeric_display(self) -> NumericPlayer {
        NumericPlayer(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn player_iter() {
        assert_eq!(
            vec![Player::One, Player::Two, Player::Three],
            Player::iter().collect::<Vec<_>>()
        )
    }
}
