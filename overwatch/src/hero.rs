use enum_iterator::IntoEnumIterator;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt;

const NUM_HEROS: usize = 31;
lazy_static! {
    pub static ref HEROPOOL: HashSet<Hero> = Hero::into_enum_iter().collect();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, IntoEnumIterator)]
pub enum Hero {
    Ana,
    Ashe,
    Baptiste,
    Bastion,
    Brigitte,
    Dva,
    Doomfist,
    Genji,
    Hanzo,
    Junkrat,
    Lucio,
    Mccree,
    Mei,
    Mercy,
    Moira,
    Orisa,
    Pharah,
    Reaper,
    Reinhardt,
    Roadhog,
    Sigma,
    Soldier76,
    Sombra,
    Symmetra,
    Torbjorn,
    Tracer,
    Widowmaker,
    Winston,
    WreckingBall,
    Zarya,
    Zenyatta,
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type HeroPool = HashSet<Hero>;

impl Hero {
    pub fn iter() -> impl Iterator<Item = Hero> {
        Self::into_enum_iter()
    }

    pub fn blizzard_name(self) -> &'static str {
        match self {
            Hero::Ana => "ana",
            Hero::Ashe => "ashe",
            Hero::Baptiste => "baptiste",
            Hero::Bastion => "bastion",
            Hero::Brigitte => "brigitte",
            Hero::Dva => "dva",
            Hero::Doomfist => "doomfist",
            Hero::Genji => "genji",
            Hero::Hanzo => "hanzo",
            Hero::Junkrat => "junkrat",
            Hero::Lucio => "lucio",
            Hero::Mccree => "mccree",
            Hero::Mei => "mei",
            Hero::Mercy => "mercy",
            Hero::Moira => "moira",
            Hero::Orisa => "orisa",
            Hero::Pharah => "pharah",
            Hero::Reaper => "reaper",
            Hero::Reinhardt => "reinhardt",
            Hero::Roadhog => "roadhog",
            Hero::Sigma => "sigma",
            Hero::Soldier76 => "soldier-76",
            Hero::Sombra => "sombra",
            Hero::Symmetra => "symmetra",
            Hero::Torbjorn => "torbjorn",
            Hero::Tracer => "tracer",
            Hero::Widowmaker => "widowmaker",
            Hero::Winston => "winston",
            Hero::WreckingBall => "wrecking-ball",
            Hero::Zarya => "zarya",
            Hero::Zenyatta => "zenyatta",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hero, HEROPOOL, NUM_HEROS};

    #[test]
    fn hero_pool() {
        assert!(HEROPOOL.contains(&Hero::Mercy));
        assert_eq!(HEROPOOL.len(), NUM_HEROS);
    }
}
