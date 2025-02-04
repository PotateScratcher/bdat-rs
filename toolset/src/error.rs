use std::fmt::Display;

use bdat::{Label, ValueType};

pub const MAX_DUPLICATE_COLUMNS: usize = 4;

#[derive(Debug)]
pub struct OptLabel(Option<Label>);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing required argument: {0}")]
    MissingRequiredArgument(&'static str),
    #[error("Unsupported file type '{0}'")]
    UnknownFileType(String),
    #[error("Not a legacy BDAT file")]
    NotLegacy,
    #[error("Not a modern BDAT file")]
    NotModern,
    #[error("No schema files found, please run 'extract' without '--no-schema'")]
    DeserMissingSchema,
    #[error(
        "Outdated schema for file '{}', found version {}, expected version {}, \
        please run 'extract' again without '--no-schema'", _0.0, _0.1, _0.2
    )]
    DeserOutdatedSchema(Box<(String, usize, usize)>),
    #[error("Table {0} is missing type information, please run 'extract' without '-u', or add types manually")]
    DeserMissingTypeInfo(OptLabel),
    #[error("Row {0} does not have entries for all columns")]
    DeserIncompleteRow(usize),
    #[error("Column {} in table {} exceeds the maximum duplicate column count of \
    {MAX_DUPLICATE_COLUMNS}. Please avoid using multiple columns with the same name.",
    _0.1, _0.0)]
    DeserMaxDuplicateColumns(Box<(OptLabel, OptLabel)>),
    #[error("Columns {} in table {} differ in type ({:?} / {:?}). \
    Please avoid using multiple columns with the same name.", 
    _0.1, _0.0, _0.2, _0.3)]
    DeserDuplicateMismatch(Box<(OptLabel, OptLabel, ValueType, ValueType)>),
}

impl Display for OptLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(l) => l.fmt(f),
            None => write!(f, "<Unnamed>"),
        }
    }
}

impl<L> From<L> for OptLabel
where
    L: Into<Option<Label>>,
{
    fn from(label: L) -> Self {
        Self(label.into())
    }
}
