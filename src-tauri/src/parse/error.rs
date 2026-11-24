use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

/// Why a log failed to load.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LogError {
    #[error("could not read file: {0}")]
    Io(String),
    #[error("missing frontmatter, file must start with `---`")]
    MissingFrontmatter,
    #[error("frontmatter is never closed with `---`")]
    UnclosedFrontmatter,
    /// Frontmatter isn't valid YAML, doesn't match the [`LogMetadata`](super::log::LogMetadata) shape, or fails a value check.
    #[error("{0}")]
    Parse(String),
}

impl LogError {
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

impl Serialize for LogError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("LogError", 2)?;
        state.serialize_field("kind", self.kind())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
