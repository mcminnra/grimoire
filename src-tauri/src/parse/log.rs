use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// A fully valid game log. Deserialized from frontmatter, serialized to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Log {
    // NOTE: file_name and body come from the file, not the frontmatter, so we skip and set manually.
    #[serde(skip_deserializing)]
    pub file_name: String, // NOTE: This is a log's unique key other systems will pin to. e.g. `007-first-light.md`
    pub title: String,
    pub tags: Option<Vec<String>>,
    #[serde(default, rename(deserialize = "log"))]
    pub details: LogDetails,
    #[serde(default)]
    pub provider_ids: ProviderIds,
    #[serde(skip_deserializing)]
    pub body: String,
}

// ============================================================================
// LogDetails and field validation logic
// ============================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct LogDetails {
    pub status: Option<Status>,
    pub rating: Option<Rating>,
    pub played_platform: Option<String>,
    pub started: Option<Date>,
    pub finished: Option<Date>,
    pub achievement_percent: Option<f64>,
    pub hours_played: Option<f64>,
    pub revisit: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Backlog,
    Playing,
    Played,
    Abandoned,
    Completed,
    #[serde(rename = "completed+")]
    CompletedPlus,
    Mastered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct Rating(u8);

/// A rating outside 1 to 5.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("rating must be 1 to 5, found {0}")]
pub struct InvalidRating(u8);

impl Rating {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 5;

    /// The rating as a plain number.
    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Rating {
    type Error = InvalidRating;

    /// Accepts only `Rating::MIN..=Rating::MAX`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidRating(value))
        }
    }
}

impl From<Rating> for u8 {
    fn from(rating: Rating) -> Self {
        rating.0
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ProviderIds {
    pub steam: Option<u64>,
    pub igdb: Option<u64>,
}
