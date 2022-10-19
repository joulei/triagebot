//! The decision process command parser.
//!
//! This can parse arbitrary input, giving the user to be assigned.
//!
//! The grammar is as follows:
//!
//! ```text
//! Command: `@bot merge`, `@bot hold`, `@bot restart`, `@bot dissent`, `@bot stabilize` or `@bot close`.
//! ```

use crate::token::{Tokenizer};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use postgres_types::{FromSql, ToSql};

/// A command as parsed and received from calling the bot with some arguments,
/// like `@rustbot merge`
#[derive(Debug, Eq, PartialEq)]
pub struct DecisionCommand {
    pub resolution: Resolution,
    pub reversibility: Reversibility
}

impl DecisionCommand {
    pub fn parse<'a>(_input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        Ok(Some(Self {
            resolution: Resolution::Merge,
            reversibility: Reversibility::Reversible
        }))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidFirstCommand
}

#[derive(Serialize, Deserialize, Debug, ToSql, FromSql, Eq, PartialEq)]
#[postgres(name = "reversibility")]
pub enum Reversibility {
    #[postgres(name = "reversible")]
    Reversible,
    #[postgres(name = "irreversible")]
    Irreversible,
}

#[derive(Serialize, Deserialize, Debug, ToSql, FromSql, Eq, PartialEq)]
#[postgres(name = "resolution")]
pub enum Resolution {
    #[postgres(name = "merge")]
    Merge,
    #[postgres(name = "hold")]
    Hold,
}
