use crate::state::State::RosterSelect;
use overwatch::overwatch_3v3::{CompBuilder, Match, Player, Roster};
use overwatch::{BattleTag, HeroPool};
use std::mem;

#[derive(Debug)]
pub enum UiEvent {
    RecordBattletag(String),
    EnterBattleTag,
    RemoveFromRoster(Player),
    RosterPlay,
}

#[derive(Debug)]
pub enum State {
    Dummy,
    RosterSelect(RosterSelectState),
    Match(MatchState),
    Exit,
}

#[derive(Debug, Default)]
pub struct RosterSelectState {
    pub battletag: String,
    pub roster: Vec<String>,
}

impl RosterSelectState {
    pub fn get_battletag(&self, player: Player) -> Option<&str> {
        self.roster.get(player.index()).map(|s| s.as_str())
    }
    #[inline]
    pub fn ready_to_play(&self) -> bool {
        self.roster.len() == 3
    }
    fn into_ow_roster(self) -> Roster {
        assert_eq!(self.roster.len(), 3);
        let mut players = self.roster.into_iter();
        Roster::new(
            players.next().unwrap(),
            players.next().unwrap(),
            players.next().unwrap(),
        )
    }
}

#[derive(Debug, Default)]
pub struct MatchState {
    selected_player: Player,
    builder: CompBuilder,
    history: Match,
    used_heros: HeroPool,
}

impl MatchState {
    fn new(roster: Roster) -> MatchState {
        MatchState {
            builder: CompBuilder::new(roster),
            ..MatchState::default()
        }
    }
}

impl State {
    pub fn new() -> State {
        State::RosterSelect(RosterSelectState::default())
    }

    pub fn transform(&mut self, updates: impl Iterator<Item = UiEvent>) {
        for e in updates {
            self.event(e)
        }
    }

    fn transition_roster_match(&mut self) {
        let mut state = State::Dummy;
        mem::swap(&mut state, self);
        let roster_state = match state {
            State::RosterSelect(r) => r,
            s => panic!(
                "attempted invalid state transition from roster -> match: {:#?}",
                s
            ),
        };
        state = State::Match(MatchState::new(roster_state.into_ow_roster()));
        mem::swap(&mut state, self);
    }

    fn event(&mut self, event: UiEvent) {
        match event {
            UiEvent::RecordBattletag(s) => {
                if let State::RosterSelect(ref mut roster_state) = self {
                    roster_state.battletag = s;
                }
            },
            UiEvent::EnterBattleTag => {
                if let State::RosterSelect(ref mut roster_state) = self {
                    if roster_state.roster.len() < 3 {
                        let mut bt = String::new();
                        mem::swap(&mut bt, &mut roster_state.battletag);
                        roster_state.roster.push(bt);
                    }
                }
            },
            UiEvent::RemoveFromRoster(p) => {
                if let State::RosterSelect(ref mut roster_state) = self {
                    let ridx = p.index();
                    if roster_state.roster.len() > ridx {
                        roster_state.roster.remove(p.index());
                    }
                }
            },
            UiEvent::RosterPlay => {
                self.transition_roster_match();
            },
        }
    }
}
