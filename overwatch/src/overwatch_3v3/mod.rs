use crate::{BattleTag, Hero, HeroPool};
use std::collections::HashSet;

pub use self::player::Player;
pub use self::roster::Roster;
pub use self::round::Round;

mod player;

mod round;

mod roster {
    use crate::BattleTag;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Roster(pub BattleTag, pub BattleTag, pub BattleTag);

    impl Roster {
        pub fn new<P1: Into<BattleTag>, P2: Into<BattleTag>, P3: Into<BattleTag>>(
            p1: P1,
            p2: P2,
            p3: P3,
        ) -> Roster {
            Roster(p1.into(), p2.into(), p3.into())
        }
    }

    impl Default for Roster {
        fn default() -> Roster {
            Roster::new("Player1", "Player2", "Player3")
        }
    }
}

#[derive(Debug, Fail)]
pub enum MatchHistoryError {
    #[fail(display = "Incomplete round information for player{}({})", _1, _0)]
    MissingPlayerHero(BattleTag, Player),
    #[fail(display = "Unknown outcome for match")]
    MissingOutcome,
    #[fail(display = "Duplicate hero in match: {}", _0)]
    DuplicateHero(Hero),
    #[fail(display = "Matches can not last longer than 5 rounds")]
    TooManyRounds,
}

#[derive(Debug)]
pub struct CompBuilder {
    player1: (BattleTag, Option<Hero>),
    player2: (BattleTag, Option<Hero>),
    player3: (BattleTag, Option<Hero>),
    win: Option<bool>,
}

impl From<Round> for CompBuilder {
    fn from(r: Round) -> CompBuilder {
        let Round {
            player1: (bt1, h1),
            player2: (bt2, h2),
            player3: (bt3, h3),
            win: win,
        } = r;

        CompBuilder {
            player1: (bt1, Some(h1)),
            player2: (bt2, Some(h2)),
            player3: (bt3, Some(h3)),
            win: Some(win),
        }
    }
}

impl Default for CompBuilder {
    fn default() -> CompBuilder {
        let r = Roster::default();
        CompBuilder::new(r)
    }
}

impl CompBuilder {
    pub fn new(roster: Roster) -> CompBuilder {
        CompBuilder {
            player1: (roster.0, None),
            player2: (roster.1, None),
            player3: (roster.2, None),
            win: None,
        }
    }

    pub fn roster(&self) -> Roster {
        Roster(
            self.player1.0.clone(),
            self.player2.0.clone(),
            self.player3.0.clone(),
        )
    }

    fn mut_player(&mut self, player: Player) -> &mut (BattleTag, Option<Hero>) {
        match player {
            Player::One => &mut self.player1,
            Player::Two => &mut self.player2,
            Player::Three => &mut self.player3,
        }
    }

    fn get_player(&self, player: Player) -> &(BattleTag, Option<Hero>) {
        match player {
            Player::One => &self.player1,
            Player::Two => &self.player2,
            Player::Three => &self.player3,
        }
    }

    pub fn set_player(&mut self, player: Player, hero: Hero) {
        self.mut_player(player).1 = Some(hero)
    }

    pub fn get_hero(&self, player: Player) -> Option<Hero> {
        self.get_player(player).1.clone()
    }

    pub fn clear_hero(&mut self, player: Player) {
        self.mut_player(player).1 = None
    }

    pub fn get_battletag(&self, player: Player) -> &BattleTag {
        &self.get_player(player).0
    }

    pub fn set_win(&mut self, win: bool) {
        self.win = Some(win)
    }

    pub fn clear_win(&mut self) {
        self.win = None
    }

    pub fn get_win(&self) -> Option<bool> {
        self.win
    }

    pub fn validate(&self) -> bool {
        for p in Player::iter() {
            if self.get_hero(p).is_none() {
                return false;
            }
        }
        self.win.is_some()
    }

    pub fn finalize(self) -> Result<Round, MatchHistoryError> {
        let win = self.win.ok_or_else(|| MatchHistoryError::MissingOutcome)?;

        let CompBuilder {
            player1: (b1, h1),
            player2: (b2, h2),
            player3: (b3, h3),
            ..
        } = self;

        Ok(Round {
            player1: finalize_player(Player::One, b1, h1)?,
            player2: finalize_player(Player::Two, b2, h2)?,
            player3: finalize_player(Player::Three, b3, h3)?,
            win,
        })
    }
}

fn finalize_player(
    player: Player,
    battletag: BattleTag,
    hero: Option<Hero>,
) -> Result<(BattleTag, Hero), MatchHistoryError> {
    if let Some(h) = hero {
        Ok((battletag, h))
    } else {
        Err(MatchHistoryError::MissingPlayerHero(battletag, player))
    }
}

#[derive(Debug, Default)]
pub struct Match {
    rounds: Vec<Round>,
}

impl Match {
    pub fn used_heros(&self) -> HeroPool {
        self.rounds
            .iter()
            .filter_map(|r| if r.win { Some(r.iter()) } else { None })
            .flatten()
            .collect()
    }

    pub fn insert_round(&mut self, round: Round) -> Result<(), MatchHistoryError> {
        if self.rounds.len() > 4 {
            return Err(MatchHistoryError::TooManyRounds);
        }
        self.get_duplicate(&round)?;
        self.rounds.push(round);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Round> {
        self.rounds.iter()
    }

    pub fn len(&self) -> usize {
        self.rounds.len()
    }

    pub fn match_outcome(&self) -> Option<bool> {
        let mut wins = 0;
        let mut loss = 0;
        for r in &self.rounds {
            if r.win {
                wins += 1;
            } else {
                loss += 1;
            }
            if wins >= 3 {
                return Some(true);
            } else if loss >= 3 {
                return Some(false);
            }
        }
        None
    }

    fn get_duplicate(&self, round: &Round) -> Result<(), MatchHistoryError> {
        self.rounds
            .iter()
            .filter_map(|r| if r.win { Some(r.iter()) } else { None })
            .flatten()
            .chain(round.iter())
            .try_fold(HashSet::new(), |mut uniq, h: Hero| {
                if !uniq.insert(h) {
                    Err(MatchHistoryError::DuplicateHero(h))
                } else {
                    Ok(uniq)
                }
            })
            .map(|_hp| ())
    }
}

#[cfg(test)]
mod tests {
    use crate::overwatch_3v3::Player;
    use crate::overwatch_3v3::{CompBuilder, Match, MatchHistoryError, Round};
    use crate::{BattleTag, Hero};

    fn catch_duplicate(r: Result<(), MatchHistoryError>, hero: Hero) {
        match r {
            Ok(_) => panic!("duplicate not detected!"),
            Err(MatchHistoryError::DuplicateHero(h)) => assert_eq!(h, hero),
            Err(e) => Err(e).expect("incorrect error type"),
        }
    }

    #[test]
    fn no_duplicate_two_rounds() {
        let mut m = Match { rounds: vec![] };
        let mut rb1 = CompBuilder::default();
        rb1.set_player(Player::One, Hero::Mercy);
        rb1.set_player(Player::Two, Hero::Pharah);
        rb1.set_player(Player::Three, Hero::Soldier76);
        rb1.set_win(true);

        let r1 = rb1.finalize().unwrap();
        m.insert_round(r1).unwrap();

        let mut rb2 = CompBuilder::default();
        rb2.set_player(Player::One, Hero::Roadhog);
        rb2.set_player(Player::Two, Hero::Mei);
        rb2.set_player(Player::Three, Hero::Brigitte);
        rb2.set_win(true);
        let r2 = rb2.finalize().unwrap();
        m.insert_round(r2).unwrap();
    }

    #[test]
    fn duplicate_in_single_winning_round() {
        let mut m = Match { rounds: vec![] };
        let mut rb1 = CompBuilder::default();
        rb1.set_player(Player::One, Hero::Mercy);
        rb1.set_player(Player::Two, Hero::Pharah);
        rb1.set_player(Player::Three, Hero::Pharah);
        rb1.set_win(true);

        let r1 = rb1.finalize().unwrap();
        catch_duplicate(m.insert_round(r1), Hero::Pharah)
    }

    #[test]
    fn duplicate_in_single_losing_round() {
        let mut m = Match { rounds: vec![] };
        let mut rb1 = CompBuilder::default();
        rb1.set_player(Player::One, Hero::Mercy);
        rb1.set_player(Player::Two, Hero::Pharah);
        rb1.set_player(Player::Three, Hero::Pharah);
        rb1.set_win(false);

        let r1 = rb1.finalize().unwrap();
        catch_duplicate(m.insert_round(r1), Hero::Pharah)
    }

    #[test]
    fn no_duplicate_in_two_losing_rounds() {
        let mut m = Match { rounds: vec![] };
        let mut rb1 = CompBuilder::default();
        rb1.set_player(Player::One, Hero::Mercy);
        rb1.set_player(Player::Two, Hero::Pharah);
        rb1.set_player(Player::Three, Hero::Soldier76);
        rb1.set_win(false);
        let r1 = rb1.finalize().unwrap();
        m.insert_round(r1).unwrap();

        let mut rb2 = CompBuilder::default();
        rb2.set_player(Player::One, Hero::Mercy);
        rb2.set_player(Player::Two, Hero::Pharah);
        rb2.set_player(Player::Three, Hero::Soldier76);
        rb2.set_win(false);
        let r2 = rb2.finalize().unwrap();
        m.insert_round(r2).unwrap();
    }

    #[test]
    fn match_used() {
        let m = Match {
            rounds: vec![
                Round {
                    player1: (BattleTag::new("a"), Hero::Ana),
                    player2: (BattleTag::new("a"), Hero::Zenyatta),
                    player3: (BattleTag::new("a"), Hero::Sigma),
                    win: false,
                },
                Round {
                    player1: (BattleTag::new("a"), Hero::Roadhog),
                    player2: (BattleTag::new("a"), Hero::Brigitte),
                    player3: (BattleTag::new("a"), Hero::Mei),
                    win: true,
                },
                Round {
                    player1: (BattleTag::new("a"), Hero::Pharah),
                    player2: (BattleTag::new("a"), Hero::Soldier76),
                    player3: (BattleTag::new("a"), Hero::Mercy),
                    win: true,
                },
            ],
        };
        assert_eq!(
            m.used_heros(),
            [
                Hero::Roadhog,
                Hero::Brigitte,
                Hero::Mei,
                Hero::Pharah,
                Hero::Soldier76,
                Hero::Mercy
            ]
            .iter()
            .cloned()
            .collect()
        )
    }
}
