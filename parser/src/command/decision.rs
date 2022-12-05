//! The decision process command parser.
//!
//! This can parse arbitrary input, giving the user to be assigned.
//!
//! The grammar is as follows:
//!
//! ```text
//! Command: `@bot merge`, `@bot hold`, `@bot restart`, `@bot dissent`, `@bot stabilize` or `@bot close`.
//! ```

use std::fmt;

use crate::error::Error;
use crate::token::{Token, Tokenizer};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

/// A command as parsed and received from calling the bot with some arguments,
/// like `@rustbot merge`
#[derive(Debug, Eq, PartialEq)]
pub struct DecisionCommand {
    pub resolution: Resolution,
    pub reversibility: Reversibility,
}

impl DecisionCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut toks = input.clone();

        match toks.peek_token()? {
            Some(Token::Word("merge")) => {
                toks.next_token()?;

                if is_token_eol(toks.peek_token()?) {
                    toks.next_token()?;
                    *input = toks;
                    return Ok(Some(Self {
                        resolution: Resolution::Merge,
                        reversibility: Reversibility::Reversible,
                    }));
                } else {
                    return Err(toks.error(ParseError::ExpectedEnd));
                }
            }
            Some(Token::Word("hold")) => {
                toks.next_token()?;

                if is_token_eol(toks.peek_token()?) {
                    toks.next_token()?;
                    *input = toks;
                    return Ok(Some(Self {
                        resolution: Resolution::Hold,
                        reversibility: Reversibility::Reversible,
                    }));
                } else {
                    return Err(toks.error(ParseError::ExpectedEnd));
                }
            }
            _ => Ok(None),
        }
    }
}

fn is_token_eol<'a>(token: Option<Token>) -> bool {
    if let Some(Token::Dot) | Some(Token::EndOfLine) = token {
        true
    } else {
        false
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    ExpectedEnd,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ExpectedEnd => write!(f, "expected end of command"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSql, FromSql, Eq, PartialEq)]
#[postgres(name = "reversibility")]
pub enum Reversibility {
    #[postgres(name = "reversible")]
    Reversible,
    #[postgres(name = "irreversible")]
    Irreversible,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSql, FromSql, Eq, PartialEq)]
#[postgres(name = "resolution")]
pub enum Resolution {
    #[postgres(name = "merge")]
    Merge,
    #[postgres(name = "hold")]
    Hold,
}
#[cfg(test)]
mod tests {
    use super::*;

    fn parse<'a>(input: &'a str) -> Result<Option<DecisionCommand>, Error<'a>> {
        let mut toks = Tokenizer::new(input);
        Ok(DecisionCommand::parse(&mut toks)?)
    }

    #[test]
    fn test_correct_merge() {
        assert_eq!(
            parse("merge"),
            Ok(Some(DecisionCommand {
                resolution: Resolution::Merge,
                reversibility: Reversibility::Reversible
            })),
        );
    }

    #[test]
    fn test_correct_merge_final_dot() {
        assert_eq!(
            parse("merge."),
            Ok(Some(DecisionCommand {
                resolution: Resolution::Merge,
                reversibility: Reversibility::Reversible
            })),
        );
    }

    #[test]
    fn test_correct_hold() {
        assert_eq!(
            parse("hold"),
            Ok(Some(DecisionCommand {
                resolution: Resolution::Hold,
                reversibility: Reversibility::Reversible
            })),
        );
    }

    #[test]
    fn test_expected_end() {
        use std::error::Error;
        assert_eq!(
            parse("hold my beer")
                .unwrap_err()
                .source()
                .unwrap()
                .downcast_ref(),
            Some(&ParseError::ExpectedEnd),
        );
    }
}
