use std::io;

use derive_more::{Display, Error, From, IsVariant};

mod args;

pub use args::*;

#[derive(Debug, Display, From, Error, IsVariant)]
pub enum CliError {
    #[display("no input provided")]
    NoInput,
    #[display("interrupted")]
    Interrupted,
    Io(io::Error),
    Generic(Box<dyn std::error::Error>),
}
