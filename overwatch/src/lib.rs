#[macro_use]
extern crate failure;

mod hero;

pub use battletag::BattleTag;
pub use hero::{Hero, HeroPool, HEROPOOL};

mod battletag {
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct BattleTag {
        inner: String,
    }

    impl<S: Into<String>> From<S> for BattleTag {
        fn from(s: S) -> BattleTag {
            BattleTag { inner: s.into() }
        }
    }

    impl BattleTag {
        pub fn new<S: Into<BattleTag>>(battletag: S) -> BattleTag {
            battletag.into()
        }
        pub fn as_str(&self) -> &str {
            self.inner.as_str()
        }
    }

    impl fmt::Display for BattleTag {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.inner)
        }
    }
}

pub mod overwatch_3v3;
