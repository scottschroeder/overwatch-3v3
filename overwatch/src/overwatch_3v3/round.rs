use crate::{BattleTag, Hero};

use super::player::PlayerEnumIterator;
use crate::overwatch_3v3::Player;

#[derive(Debug)]
pub struct Round {
    pub player1: (BattleTag, Hero),
    pub player2: (BattleTag, Hero),
    pub player3: (BattleTag, Hero),
    pub win: bool,
}

impl Round {
    pub fn iter(&self) -> RoundIter {
        self.into_iter()
    }

    pub fn get_player(&self, player: Player) -> &(BattleTag, Hero) {
        match player {
            Player::One => &self.player1,
            Player::Two => &self.player2,
            Player::Three => &self.player3,
        }
    }
    pub fn get_hero(&self, player: Player) -> Hero {
        self.get_player(player).1
    }
}

impl<'a> IntoIterator for &'a Round {
    type Item = Hero;
    type IntoIter = RoundIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RoundIter {
            player: Player::iter(),
            round: &self,
        }
    }
}

pub struct RoundIter<'a> {
    player: PlayerEnumIterator,
    round: &'a Round,
}

impl<'a> Iterator for RoundIter<'a> {
    type Item = Hero;
    fn next(&mut self) -> Option<Hero> {
        self.player.next().map(|p| {
            let hero = match p {
                Player::One => self.round.player1.1,
                Player::Two => self.round.player2.1,
                Player::Three => self.round.player3.1,
            };
            hero
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Round;
    use crate::{BattleTag, Hero};

    #[test]
    fn round_iter() {
        let r = Round {
            player1: (BattleTag::new("a"), Hero::Roadhog),
            player2: (BattleTag::new("a"), Hero::Brigitte),
            player3: (BattleTag::new("a"), Hero::Mei),
            win: false,
        };
        assert_eq!(
            vec![Hero::Roadhog, Hero::Brigitte, Hero::Mei],
            r.iter().collect::<Vec<_>>()
        )
    }
}
