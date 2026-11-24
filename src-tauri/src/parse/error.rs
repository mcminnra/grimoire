use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

/// A log file that failed to load
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedLog {
    pub file_name: String,
    /// Full file contents. `None` only when the file couldn't be read at all.
    pub source: Option<String>,
    pub error: LogLoadError,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LogLoadError {
    #[error("could not read file: {0}")]
    Io(String),
    #[error("missing frontmatter, file must start with `---`")]
    MissingFrontmatter,
    #[error("frontmatter is never closed with `---`")]
    UnclosedFrontmatter,
    /// Frontmatter isn't valid YAML, doesn't match the [`Log`](super::log::Log) shape, or fails a value check.
    #[error("{0}")]
    Parse(String),
}

impl LogLoadError {
    /// Stable camelCase name of the variant for the frontend.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Io(_) => "io",
            Self::MissingFrontmatter => "missingFrontmatter",
            Self::UnclosedFrontmatter => "unclosedFrontmatter",
            Self::Parse(_) => "parse",
        }
    }
}

impl Serialize for LogLoadError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("LogLoadError", 2)?;
        state.serialize_field("kind", self.kind())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
