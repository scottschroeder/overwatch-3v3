use enum_iterator::IntoEnumIterator;
use failure::_core::str::FromStr;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt;

const NUM_HEROS: usize = 31;
lazy_static! {
    pub static ref HEROPOOL: HashSet<Hero> = Hero::into_enum_iter().collect();
}

#[derive(Debug, Fail)]
#[fail(display = "could not parse hero from '{}'", _0)]
pub struct ParseHeroError(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, IntoEnumIterator)]
pub enum Role {
    Dps,
    Tank,
    Support,
}

impl Role {
    pub fn heros(self) -> impl Iterator<Item = Hero> {
        Hero::iter().filter(move |h| h.role() == self)
    }
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

    pub fn role(self) -> Role {
        match self {
            Hero::Ana => Role::Support,
            Hero::Ashe => Role::Dps,
            Hero::Baptiste => Role::Support,
            Hero::Bastion => Role::Dps,
            Hero::Brigitte => Role::Support,
            Hero::Dva => Role::Tank,
            Hero::Doomfist => Role::Dps,
            Hero::Genji => Role::Dps,
            Hero::Hanzo => Role::Dps,
            Hero::Junkrat => Role::Dps,
            Hero::Lucio => Role::Support,
            Hero::Mccree => Role::Dps,
            Hero::Mei => Role::Dps,
            Hero::Mercy => Role::Support,
            Hero::Moira => Role::Support,
            Hero::Orisa => Role::Tank,
            Hero::Pharah => Role::Dps,
            Hero::Reaper => Role::Dps,
            Hero::Reinhardt => Role::Tank,
            Hero::Roadhog => Role::Tank,
            Hero::Sigma => Role::Tank,
            Hero::Soldier76 => Role::Dps,
            Hero::Sombra => Role::Dps,
            Hero::Symmetra => Role::Dps,
            Hero::Torbjorn => Role::Dps,
            Hero::Tracer => Role::Dps,
            Hero::Widowmaker => Role::Dps,
            Hero::Winston => Role::Tank,
            Hero::WreckingBall => Role::Tank,
            Hero::Zarya => Role::Tank,
            Hero::Zenyatta => Role::Support,
        }
    }

    /// This is the name used for loading assets
    /// and for database entries. Never change these, as
    /// they are "string-ly" typed.
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

impl FromStr for Hero {
    type Err = ParseHeroError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ana" => Hero::Ana,
            "ashe" => Hero::Ashe,
            "baptiste" => Hero::Baptiste,
            "bastion" => Hero::Bastion,
            "brigitte" => Hero::Brigitte,
            "dva" => Hero::Dva,
            "doomfist" => Hero::Doomfist,
            "genji" => Hero::Genji,
            "hanzo" => Hero::Hanzo,
            "junkrat" => Hero::Junkrat,
            "lucio" => Hero::Lucio,
            "mccree" => Hero::Mccree,
            "mei" => Hero::Mei,
            "mercy" => Hero::Mercy,
            "moira" => Hero::Moira,
            "orisa" => Hero::Orisa,
            "pharah" => Hero::Pharah,
            "reaper" => Hero::Reaper,
            "reinhardt" => Hero::Reinhardt,
            "roadhog" => Hero::Roadhog,
            "sigma" => Hero::Sigma,
            "soldier-76" => Hero::Soldier76,
            "sombra" => Hero::Sombra,
            "symmetra" => Hero::Symmetra,
            "torbjorn" => Hero::Torbjorn,
            "tracer" => Hero::Tracer,
            "widowmaker" => Hero::Widowmaker,
            "winston" => Hero::Winston,
            "wrecking-ball" => Hero::WreckingBall,
            "zarya" => Hero::Zarya,
            "zenyatta" => Hero::Zenyatta,
            s => return Err(ParseHeroError(s.into())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Hero, ParseHeroError, HEROPOOL, NUM_HEROS};
    use std::str::FromStr;

    #[test]
    fn hero_pool() {
        assert!(HEROPOOL.contains(&Hero::Mercy));
        assert_eq!(HEROPOOL.len(), NUM_HEROS);
    }

    #[test]
    fn parse_all_heros() {
        for hero in Hero::iter() {
            let s = hero.blizzard_name();
            let parsed_hero = Hero::from_str(s).unwrap();
            assert_eq!(hero, parsed_hero);
        }
    }

    #[test]
    fn parse_hero_failure() {
        let bad_hero_name = "pharmacy";
        match Hero::from_str(bad_hero_name) {
            Ok(h) => panic!("no hero named: {}", h),
            Err(ParseHeroError(s)) => assert_eq!(s.as_str(), bad_hero_name),
        }
    }
}
