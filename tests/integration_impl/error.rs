use super::test_highlight;
use crate::integration_impl::bindings::QueryError;
use std::fmt::{self, Display, Write};
use std::io;

#[derive(Debug)]
pub struct Error(pub Vec<String>);

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn grammar(message: &str) -> Self {
        Error(vec![format!("Grammar error: {}", message)])
    }

    pub fn regex(message: &str) -> Self {
        Error(vec![format!("Regex error: {}", message)])
    }

    pub fn undefined_symbol(name: &str) -> Self {
        Error(vec![format!("Undefined symbol `{}`", name)])
    }

    pub fn new(message: String) -> Self {
        Error(vec![message])
    }

    pub fn err<T>(message: String) -> Result<T> {
        Err(Error::new(message))
    }

    pub fn wrap<E: Into<Self>, M: ToString, F: FnOnce() -> M>(
        message_fn: F,
    ) -> impl FnOnce(E) -> Self {
        |e| {
            let mut result = e.into();
            result.0.push(message_fn().to_string());
            result
        }
    }

    pub fn message(&self) -> String {
        let mut result = self.0.last().unwrap().clone();
        if self.0.len() > 1 {
            result.push_str("\nDetails:\n");
            for msg in self.0[0..self.0.len() - 1].iter().rev() {
                writeln!(&mut result, "  {}", msg).unwrap();
            }
        }
        result
    }
}

impl<'a> From<QueryError> for Error {
    fn from(error: QueryError) -> Self {
        match error {
            QueryError::Capture(row, c) => Error::new(format!(
                "Query error on line {}: Invalid capture name {}",
                row, c
            )),
            QueryError::Field(row, f) => Error::new(format!(
                "Query error on line {}: Invalid field name {}",
                row, f
            )),
            QueryError::NodeType(row, t) => Error::new(format!(
                "Query error on line {}. Invalid node type {}",
                row, t
            )),
            QueryError::Syntax(row, l) => Error::new(format!(
                "Query error on line {}. Invalid syntax:\n{}",
                row, l
            )),
            QueryError::Predicate(p) => Error::new(format!("Query error: {}", p)),
        }
    }
}

impl<'a> From<crate::integration_impl::tree_sitter_highlight::Error> for Error {
    fn from(error: crate::integration_impl::tree_sitter_highlight::Error) -> Self {
        Error::new(format!("{:?}", error))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::new(error.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::new(error.to_string())
    }
}

impl From<glob::PatternError> for Error {
    fn from(error: glob::PatternError) -> Self {
        Error::new(error.to_string())
    }
}

impl From<glob::GlobError> for Error {
    fn from(error: glob::GlobError) -> Self {
        Error::new(error.to_string())
    }
}

impl From<regex_syntax::ast::Error> for Error {
    fn from(error: regex_syntax::ast::Error) -> Self {
        Error::new(error.to_string())
    }
}

impl From<test_highlight::Failure> for Error {
    fn from(error: test_highlight::Failure) -> Self {
        Error::new(error.message())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::new(error)
    }
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Error::new(error.message().to_string())
    }
}

impl From<walkdir::Error> for Error {
    fn from(error: walkdir::Error) -> Self {
        Error::new(error.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => write!(f, "ERROR: unknown error"),
            1 => write!(f, "ERROR: {}", &self.0[0]),
            _ => {
                writeln!(f, "ERROR:")?;
                self.0
                    .iter()
                    .try_for_each(|error| writeln!(f, "\t{}", error))
            }
        }
    }
}

impl std::error::Error for Error {}
