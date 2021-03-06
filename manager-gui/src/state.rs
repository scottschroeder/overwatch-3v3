use overwatch::overwatch_3v3::{CompBuilder, Match, Player, Roster, Round};
use overwatch::{BattleTag, Hero, HeroPool};
use std::mem;
use match_history::{MatchDb, MatchDbError, open};
use std::path::PathBuf;

#[derive(Debug)]
pub enum UiEvent {
    OpenDatabase(PathBuf),
    RecordBattletag(String),
    EnterBattleTag,
    RemoveFromRoster(Player),
    RosterPlay,
    RoundSelectPlayer(Player),
    RoundSelectHero(Hero),
    RoundToggleOutcome,
    RoundRecord,
}

#[derive(Debug)]
pub enum State {
    Dummy,
    LoadDatabase(LoadDbState),
    RosterSelect(RosterSelectState, MatchDb),
    Match(MatchState, MatchDb),
    Exit,
}

#[derive(Debug)]
pub enum LoadDbState {
    NotReady,
    Ready(MatchDb),
    Failure(MatchDbError),
}

impl LoadDbState {
    fn open(&mut self, path: PathBuf) {
        let mut new = match open(&path) {
            Ok(mdb) =>  LoadDbState::Ready(mdb),
            Err(e) => LoadDbState::Failure(e),
        };
        mem::swap(self, &mut new)
    }
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

    pub fn match_len(&self) -> usize {
        self.history.len()
    }

    pub fn match_iter(&self) -> impl Iterator<Item=&Round> {
        self.history.iter()
    }

    #[inline]
    pub fn is_used(&self, hero: Hero) -> bool {
        self.used_heros.contains(&hero)
    }

    #[inline]
    pub fn get_battletag(&self, player: Player) -> &BattleTag {
        self.builder.get_battletag(player)
    }

    #[inline]
    pub fn get_hero(&self, player: Player) -> Option<Hero> {
        self.builder.get_hero(player)
    }

    #[inline]
    pub fn get_selected_player(&self) -> Player {
        self.selected_player
    }

    #[inline]
    pub fn get_win(&self) -> Option<bool> {
        self.builder.get_win()
    }

    #[inline]
    pub fn validate(&self) -> bool {
        self.builder.validate()
    }

    fn clear_hero_selection(&mut self, player: Player) {
        if let Some(hero) = self.builder.get_hero(player) {
            self.used_heros.remove(&hero);
            self.builder.clear_hero(player);
        }
    }

    fn select_hero(&mut self, hero: Hero) {
        if self.used_heros.contains(&hero) {
            return;
        }
        self.clear_hero_selection(self.selected_player);
        self.builder.set_player(self.selected_player, hero);
        self.used_heros.insert(hero);
        self.selected_player = self.selected_player.cycle_next();
    }
}

impl State {
    pub fn new() -> State {
        State::LoadDatabase(LoadDbState::NotReady)
    }

    pub fn transform(&mut self, updates: impl Iterator<Item=UiEvent>) {
        for e in updates {
            self.event(e)
        }
    }

    fn transition_loaddb_roster(&mut self) {
        let mut state = State::Dummy;
        mem::swap(&mut state, self);
        let db = match state {
            State::LoadDatabase( LoadDbState::Ready(db)) =>  db,
            s => panic!(
                "attempted invalid state transition from match -> roster: {:#?}",
                s
            ),
        };

        state = State::RosterSelect(RosterSelectState {
            battletag: "".to_string(),
            roster: Vec::new(),
        }, db);
        mem::swap(&mut state, self);
    }

    fn transition_roster_match(&mut self) {
        let mut state = State::Dummy;
        mem::swap(&mut state, self);
        let (roster_state, db) = match state {
            State::RosterSelect(r, db) => (r, db),
            s => panic!(
                "attempted invalid state transition from roster -> match: {:#?}",
                s
            ),
        };
        state = State::Match(MatchState::new(roster_state.into_ow_roster()), db);
        mem::swap(&mut state, self);
    }

    fn transition_match_roster(&mut self) {
        let mut state = State::Dummy;
        mem::swap(&mut state, self);
        let (match_state, db) = match state {
            State::Match(m, db) => (m, db),
            s => panic!(
                "attempted invalid state transition from match -> roster: {:#?}",
                s
            ),
        };

        let MatchState { builder, ..} = match_state;

        let Roster(p1, p2, p3) = builder.roster();
        state = State::RosterSelect(RosterSelectState {
            battletag: "".to_string(),
            roster: vec![p1.into_inner(), p2.into_inner(), p3.into_inner()],
        }, db);
        mem::swap(&mut state, self);
    }

    pub fn event(&mut self, event: UiEvent) {
        match event {
            UiEvent::OpenDatabase(path) => {
                if let State::LoadDatabase(ref mut load_db_state) = self {
                    load_db_state.open(path);
                    if let LoadDbState::Ready(db) = load_db_state {
                        self.transition_loaddb_roster()
                    }
                }
            }
            UiEvent::RecordBattletag(s) => {
                if let State::RosterSelect(ref mut roster_state, _) = self {
                    roster_state.battletag = s;
                }
            }
            UiEvent::EnterBattleTag => {
                if let State::RosterSelect(ref mut roster_state,_) = self {
                    if roster_state.roster.len() < 3 {
                        let mut bt = String::new();
                        mem::swap(&mut bt, &mut roster_state.battletag);
                        roster_state.roster.push(bt);
                    }
                }
            }
            UiEvent::RemoveFromRoster(p) => {
                if let State::RosterSelect(ref mut roster_state,_) = self {
                    let ridx = p.index();
                    if roster_state.roster.len() > ridx {
                        roster_state.roster.remove(p.index());
                    }
                }
            }
            UiEvent::RosterPlay => {
                self.transition_roster_match();
            }
            UiEvent::RoundSelectPlayer(p) => {
                if let State::Match(ref mut match_state,_) = self {
                    match_state.selected_player = p;
                    match_state.clear_hero_selection(p);
                }
            }
            UiEvent::RoundSelectHero(h) => {
                if let State::Match(ref mut match_state,_) = self {
                    match_state.select_hero(h)
                }
            }
            UiEvent::RoundToggleOutcome => {
                if let State::Match(ref mut match_state,_) = self {
                    match_state
                        .builder
                        .set_win(match match_state.builder.get_win() {
                            Some(true) => false,
                            _ => true,
                        })
                }
            }
            UiEvent::RoundRecord => {
                if let State::Match(ref mut match_state,_) = self {
                    let mut builder = CompBuilder::new(match_state.builder.roster());
                    mem::swap(&mut match_state.builder, &mut builder);
                    let r = builder.finalize().unwrap();
                    match_state.history.insert_round(r).unwrap();
                    match_state.used_heros = match_state.history.used_heros();
                    match_state.selected_player = Player::One;
                    if match_state.history.match_outcome().is_some() {
                        self.transition_match_roster()
                    }
                }
            }
        }
    }
}
