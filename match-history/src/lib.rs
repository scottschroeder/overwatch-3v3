#[macro_use]
extern crate log;

use overwatch;
use overwatch::{BattleTag, Hero};

use crate::MatchDbError::SqliteError;
use rusqlite::ffi::Error as RusqliteFfiError;
use rusqlite::types::Value;
use rusqlite::Error as RusqliteError;
use rusqlite::ErrorCode;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, OptionalExtension, Statement};

use failure::Fail;
use failure::_core::ops::Deref;
use overwatch::overwatch_3v3::{CompBuilder, Match, Player, Roster, Round};
use std::path;

const SCHEMA_TABLE_BATTLETAGS: &str = "battletags";
const SCHEMA_TABLE_MATCH: &str = "matches";
const SCHEMA_TABLE_ROUND: &str = "rounds";
const SCHEMA_TABLE_PLAYS: &str = "plays";

#[derive(Debug, Fail)]
pub enum MatchDbError {
    #[fail(display = "Battletag '{}' already exists", _0)]
    BattletagAlreadyExists(BattleTag),
    #[fail(display = "Battletag '{}' does not exist", _0)]
    BattletagDoesNotExist(BattleTag),
    #[fail(display = "Sqlite Error")]
    SqliteError(#[cause] rusqlite::Error),
}

impl From<RusqliteError> for MatchDbError {
    fn from(e: RusqliteError) -> MatchDbError {
        MatchDbError::SqliteError(e)
    }
}

#[derive(Debug)]
pub struct MatchDb {
    conn: Connection,
}

pub fn open<P: AsRef<path::Path>>(path: P) -> Result<MatchDb, MatchDbError> {
    let conn = Connection::open(path)?;
    create_schema(&conn)?;
    Ok(MatchDb::new(conn))
}

impl MatchDb {
    pub fn new(conn: Connection) -> MatchDb {
        MatchDb { conn }
    }

    pub fn get_or_insert_battletag_id(&self, battletag: &BattleTag) -> Result<i64, MatchDbError> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id from {} WHERE name = (?1)",
            SCHEMA_TABLE_BATTLETAGS
        ))?;

        let existing_id = stmt
            .query_row(&[format!("{}", battletag)], |r| {
                let s: rusqlite::Result<i64> = r.get(0);
                s
            })
            .optional()?;
        Ok(if let Some(id) = existing_id {
            id
        } else {
            self.record_battletag(battletag)?
        })
    }

    pub fn record_battletag(&self, battletag: &BattleTag) -> Result<i64, MatchDbError> {
        let sql = format!("INSERT INTO {} (name) values (?1)", SCHEMA_TABLE_BATTLETAGS);
        self.conn
            .execute(&sql, &[battletag.as_str()])
            .map_err(|e| {
                let mut new_e = None;
                if let RusqliteError::SqliteFailure(ffierr, _) = e {
                    if let ErrorCode::ConstraintViolation = ffierr.code {
                        new_e = Some(MatchDbError::BattletagAlreadyExists(battletag.clone()))
                    }
                }
                new_e.unwrap_or_else(|| e.into())
            })?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn search_battletags(&self, search: &str) -> Result<Vec<BattleTag>, MatchDbError> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT name from {} WHERE name LIKE (?1)",
            SCHEMA_TABLE_BATTLETAGS
        ))?;
        let mapping = stmt.query_map(&[format!("%{}%", search)], |r| {
            let s: rusqlite::Result<String> = r.get(0);
            s
        })?;
        mapping
            .map(|r| r.map(BattleTag::new).map_err(|e| e.into()))
            .collect()
    }

    pub fn record_match(&mut self, match_result: &Match) -> Result<(), MatchDbError> {
        let sql = format!("INSERT INTO {} DEFAULT VALUES", SCHEMA_TABLE_MATCH);
        let empty: &[&str] = &[];

        let tx = self.conn.transaction()?;
        tx.execute(&sql, empty)?;
        let match_id = tx.last_insert_rowid();
        for round in match_result.iter() {
            record_round(tx.deref(), match_id, round)?;
        }
        tx.commit()?;
        Ok(())
    }
}

fn record_round<C: Deref<Target = Connection>>(
    conn: C,
    match_id: i64,
    round: &Round,
) -> Result<(), MatchDbError> {
    let sql = format!(
        "INSERT INTO {} (match_id, is_win) VALUES (?1, ?2)",
        SCHEMA_TABLE_ROUND
    );
    conn.execute(
        &sql,
        &[Value::Integer(match_id), Value::Integer(round.win as i64)],
    )?;
    let round_id = conn.last_insert_rowid();
    for player in Player::iter() {
        let (bt, hero) = round.get_player(player);
        record_play(conn.deref(), round_id, bt, *hero)?;
    }
    Ok(())
}
fn record_play<C: Deref<Target = Connection>>(
    conn: C,
    round_id: i64,
    battletag: &BattleTag,
    hero: Hero,
) -> Result<(), MatchDbError> {
    let sql = format!(
        "\
         INSERT INTO {} ( round_id, battletag_id, hero ) VALUES (\
         ?1,\
         (SELECT id from {} WHERE name = ?2),\
         ?3\
         )",
        SCHEMA_TABLE_PLAYS, SCHEMA_TABLE_BATTLETAGS
    );
    conn.execute(
        &sql,
        &[
            Value::Integer(round_id),
            Value::Text(battletag.as_str().into()),
            Value::Text(format!("{}", hero.blizzard_name())),
        ],
    )
    .map_err(|e| {
        let mut new_e = None;
        if let RusqliteError::SqliteFailure(ffierr, _) = e {
            if let ErrorCode::ConstraintViolation = ffierr.code {
                new_e = Some(MatchDbError::BattletagDoesNotExist(battletag.clone()))
            }
        }
        new_e.unwrap_or_else(|| e.into())
    })?;
    Ok(())
}

impl Default for MatchDb {
    fn default() -> MatchDb {
        let conn = Connection::open_in_memory().unwrap();
        create_schema(&conn).unwrap();
        MatchDb { conn }
    }
}

pub fn main() -> Result<(), failure::Error> {
    let conn = Connection::open("bt.db")?;
    create_schema(&conn)?;
    let mut mdb = MatchDb::new(conn);

    let p1 = BattleTag::new("player1");
    let p2 = BattleTag::new("player2");
    let p3 = BattleTag::new("player3");

    mdb.record_battletag(&p1)?;
    mdb.record_battletag(&p2)?;
    mdb.record_battletag(&p3)?;

    let x = mdb.search_battletags("an")?;

    let mut m = Match::default();
    let mut builder = CompBuilder::new(Roster::new(p1, p2, p3));
    builder.set_player(Player::One, Hero::Mercy);
    builder.set_player(Player::Two, Hero::Torbjorn);
    builder.set_player(Player::Three, Hero::Pharah);
    builder.set_win(true);
    m.insert_round(builder.finalize()?)?;
    mdb.record_match(&m)?;

    info!("x: {:#?}", x);

    Ok(())
}

pub fn create_schema(conn: &Connection) -> Result<(), MatchDbError> {
    conn.execute(
        &format!(
            "create table if not exists {} (
             id integer primary key,
             name text not null unique
         )",
            SCHEMA_TABLE_BATTLETAGS
        ),
        NO_PARAMS,
    )?;
    conn.execute(
        &format!(
            "create table if not exists {} (
             id integer primary key,
             timestamp datetime default CURRENT_TIMESTAMP
         )",
            SCHEMA_TABLE_MATCH
        ),
        NO_PARAMS,
    )?;
    conn.execute(
        &format!(
            "create table if not exists {} (
             id integer primary key,
             match_id integer not null,
             is_win bool not null,
             FOREIGN KEY(match_id) REFERENCES {}(id)
         )",
            SCHEMA_TABLE_ROUND, SCHEMA_TABLE_MATCH
        ),
        NO_PARAMS,
    )?;
    conn.execute(
        &format!(
            "create table if not exists {} (
             id integer primary key,
             round_id integer not null,
             battletag_id integer not null,
             hero TEXT not null,
             FOREIGN KEY(round_id) REFERENCES {}(id),
             FOREIGN KEY(battletag_id) REFERENCES {}(id)
         )",
            SCHEMA_TABLE_PLAYS, SCHEMA_TABLE_ROUND, SCHEMA_TABLE_BATTLETAGS
        ),
        NO_PARAMS,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitve_battletag_search() {
        let db = MatchDb::default();
        let tags = vec![
            BattleTag::new("player1"),
            BattleTag::new("PLAYER2"),
            BattleTag::new("pLaYEr3"),
        ];
        for t in &tags {
            db.record_battletag(t).unwrap();
        }

        let lower_search = db.search_battletags("player").unwrap();
        assert_eq!(lower_search, tags);

        let upper_search = db.search_battletags("PLAYER").unwrap();
        assert_eq!(upper_search, tags);

        let mixed_search = db.search_battletags("PlAyeR").unwrap();
        assert_eq!(mixed_search, tags);
    }

    #[test]
    fn repeat_insert_battletag_custom_error() {
        let db = MatchDb::default();
        let p1 = BattleTag::new("player1");
        let p2 = BattleTag::new("player2");
        db.record_battletag(&p1).unwrap();
        db.record_battletag(&p2).unwrap();
        match db.record_battletag(&p1) {
            Ok(_) => panic!("did not error when a battletag was inserted twice"),
            Err(MatchDbError::BattletagAlreadyExists(repeat_bt)) => {
                assert_eq!(repeat_bt, p1);
            },
            Err(e) => {
                panic!("raised incorrect error: {}", e);
            },
        }
    }
}
