use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use super::error::LogError;

const DELIMITER: &str = "---";

#[derive(Debug, Clone)]
pub struct Log {
    path: PathBuf,
    file_name: String, // NOTE: This is a log's unique key other systems will pin to. e.g. `007-first-light.md`
    pub source: Option<String>,
    pub metadata: Option<LogMetadata>,
    pub body: Option<String>,
    pub error: Option<LogError>,
}

impl Log {
    /// Read and parse the log file at `path`.
    pub fn load(path: PathBuf) -> io::Result<Self> {
        let file_name = Self::key(&path)?;
        Ok(match fs::read_to_string(&path) {
            Ok(source) => Self::from_source(path, file_name, source),
            Err(e) => Self::failed(path, file_name, None, LogError::Io(e.to_string())),
        })
    }

    /// Parse `source` as the contents of the log file at `path`
    pub fn parse(path: PathBuf, source: String) -> io::Result<Self> {
        let file_name = Self::key(&path)?;
        Ok(Self::from_source(path, file_name, source))
    }

    /// Full path to the file on disk
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The log's unique key, the exact UTF-8 file name of [`Log::path`]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Constructs a valid file_name utf key
    fn key(path: &Path) -> io::Result<String> {
        path.file_name()
            .and_then(OsStr::to_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("log file name is not valid UTF-8: {}", path.display()),
                )
            })
    }

    /// Builds a log that failed to load, with no metadata or body.
    fn failed(path: PathBuf, file_name: String, source: Option<String>, error: LogError) -> Self {
        Self {
            path,
            file_name,
            source,
            metadata: None,
            body: None,
            error: Some(error),
        }
    }

    /// Parse `source` into a log keyed by `file_name`
    fn from_source(path: PathBuf, file_name: String, source: String) -> Self {
        let (frontmatter, body) = match split_frontmatter(&source) {
            Ok(split) => split,
            Err(error) => return Self::failed(path, file_name, Some(source), error),
        };
        let (metadata, error) = match parse_metadata(&frontmatter) {
            Ok(metadata) => (Some(metadata), None),
            Err(error) => (None, Some(error)),
        };

        Self {
            path,
            file_name,
            source: Some(source),
            metadata,
            body: Some(body),
            error,
        }
    }
}

/// Splits `source` on the leading `---` frontmatter fences into owned `(frontmatter, body)`.
///
/// The first line must be `---`. The frontmatter runs until the next line that is
/// exactly `---`. Everything after that line is the body.
///
/// # Errors
///
/// Returns [`LogError::MissingFrontmatter`] or [`LogError::UnclosedFrontmatter`]
/// when the opening or closing fence is missing.
fn split_frontmatter(source: &str) -> Result<(String, String), LogError> {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let mut lines = source.split_inclusive('\n');

    let opening = lines.next().unwrap_or_default();
    if opening.trim_end() != DELIMITER {
        return Err(LogError::MissingFrontmatter);
    }

    let mut frontmatter = String::new();
    for line in lines.by_ref() {
        if line.trim_end() == DELIMITER {
            let body = lines.collect::<String>().trim_start().to_string();
            return Ok((frontmatter, body));
        }
        frontmatter.push_str(line);
    }

    Err(LogError::UnclosedFrontmatter)
}

/// Deserialize frontmatter YAML into [`LogMetadata`].
///
/// # Errors
///
/// Returns [`LogError::Parse`] when the frontmatter is invalid YAML, doesn't match the
/// [`LogMetadata`] shape, or fails a value check like [`Rating`].
fn parse_metadata(frontmatter: &str) -> Result<LogMetadata, LogError> {
    // NOTE: re-adding the `---` fence keeps YAML error line numbers equal to file line numbers
    let yaml = format!("---\n{frontmatter}");
    serde_saphyr::from_str(&yaml).map_err(|e| LogError::Parse(e.to_string()))
}

// ============================================================================
// Log metdata and field validation logic
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct LogMetadata {
    pub title: String,
    pub tags: Option<Vec<String>>,
    #[serde(default, rename(deserialize = "log"))]
    pub details: LogDetails,
    #[serde(default)]
    pub provider_ids: ProviderIds,
}

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

// ============================================================================
// Log view for frontend
// ============================================================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogView {
    pub file_name: String,
    pub source: Option<String>,
    pub metadata: Option<LogMetadata>,
    pub body: Option<String>,
    pub error: Option<LogError>,
}

impl From<&Log> for LogView {
    fn from(log: &Log) -> Self {
        Self {
            file_name: log.file_name.clone(),
            source: log.source.clone(),
            metadata: log.metadata.clone(),
            body: log.body.clone(),
            error: log.error.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(frontmatter: &str) -> Log {
        let source = format!("---\n{frontmatter}---\nbody");
        Log::parse("logs/test.md".into(), source).unwrap()
    }

    /// Parses and unwraps metadata, panicking with the error if there is none.
    fn parse_ok(frontmatter: &str) -> (Log, LogMetadata) {
        let log = parse(frontmatter);
        let metadata = log
            .metadata
            .clone()
            .unwrap_or_else(|| panic!("{:?}", log.error));
        (log, metadata)
    }

    /// Parses and unwraps the error, panicking if parsing succeeded.
    fn parse_err(frontmatter: &str) -> (Log, LogError) {
        let log = parse(frontmatter);
        let error = log.error.clone().expect("expected an error");
        assert!(log.metadata.is_none());
        (log, error)
    }

    #[test]
    fn parses_full_log() {
        let (log, meta) = parse_ok(
            r#"title: 'World of Warcraft: Midnight'
tags: [mmo, blizzard]
log:
  status: completed+
  rating: 4
  played_platform: PC
  started: 2026-03-02
  finished: 2026-04-10
  achievement_percent: 42.5
  hours_played: 120
  revisit: true
provider_ids:
  steam: 12345
  igdb: 678
"#,
        );

        assert_eq!(log.path, PathBuf::from("logs/test.md"));
        assert_eq!(log.file_name, "test.md");
        assert!(log.error.is_none());
        assert_eq!(meta.title, "World of Warcraft: Midnight");
        assert_eq!(
            meta.tags.as_deref(),
            Some(&["mmo".to_string(), "blizzard".to_string()][..])
        );
        assert_eq!(meta.details.status, Some(Status::CompletedPlus));
        assert_eq!(meta.details.rating.map(Rating::get), Some(4));
        assert_eq!(meta.details.started, Some(jiff::civil::date(2026, 3, 2)));
        assert_eq!(meta.details.hours_played, Some(120.0));
        assert_eq!(meta.details.revisit, Some(true));
        assert_eq!(meta.provider_ids.steam, Some(12345));
        assert_eq!(log.body.as_deref(), Some("body"));
    }

    #[test]
    fn nulls_and_unknown_keys_are_fine() {
        let (_, meta) = parse_ok("title: Foo\ntags: null\ncover: null\nlog:\n  rating: null\n");
        assert_eq!(meta.tags, None);
        assert_eq!(meta.details.rating, None);
    }

    #[test]
    fn missing_title_is_a_parse_error() {
        let (log, error) = parse_err("tags: []\n");
        assert!(matches!(error, LogError::Parse(_)), "{error}");
        assert_eq!(log.source.as_deref(), Some("---\ntags: []\n---\nbody"));
        assert_eq!(log.body.as_deref(), Some("body"));
    }

    #[test]
    fn unknown_status_is_a_parse_error() {
        let (_, error) = parse_err("title: Foo\nlog:\n  status: beaten\n");
        assert!(matches!(error, LogError::Parse(_)), "{error}");
    }

    #[test]
    fn rating_out_of_range_is_a_parse_error_with_location() {
        for rating in ["0", "6", "-1", "300"] {
            let (_, error) = parse_err(&format!("title: Foo\nlog:\n  rating: {rating}\n"));
            let LogError::Parse(message) = &error else {
                panic!("expected parse error, got {error:?}");
            };
            assert!(message.contains("line 4"), "{message}");
        }
    }

    #[test]
    fn invalid_yaml_is_a_parse_error() {
        let (_, error) = parse_err("title: [unclosed\n");
        assert!(matches!(error, LogError::Parse(_)), "{error}");
    }

    #[test]
    fn missing_fence_keeps_only_source() {
        let log = Log::parse("logs/a.md".into(), "title: Foo\n".into()).unwrap();
        assert_eq!(log.file_name, "a.md");
        assert_eq!(log.error, Some(LogError::MissingFrontmatter));
        assert_eq!(log.source.as_deref(), Some("title: Foo\n"));
        assert!(log.metadata.is_none() && log.body.is_none());
    }

    #[test]
    fn unreadable_file_is_an_io_error() {
        let log = Log::load("/nonexistent/grimoire/a.md".into()).unwrap();
        assert!(
            matches!(log.error, Some(LogError::Io(_))),
            "{:?}",
            log.error
        );
        assert!(log.source.is_none());
    }

    #[test]
    fn key_is_exact_file_name() {
        let key = Log::key(Path::new("logs/007-first-light.md")).unwrap();
        assert_eq!(key, "007-first-light.md");
    }

    #[test]
    fn missing_file_name_is_an_error() {
        let error = Log::parse("logs/..".into(), String::new()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    // NOTE: unix only since Windows paths can't be built from raw bytes
    #[cfg(unix)]
    #[test]
    fn non_utf8_file_name_is_an_error_not_lossy() {
        use std::os::unix::ffi::OsStrExt;
        let path = Path::new("logs").join(OsStr::from_bytes(b"\xff.md"));
        let error = Log::load(path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    // NOTE: unix only since Windows paths can't be built from raw bytes
    #[cfg(unix)]
    #[test]
    fn non_utf8_directory_is_fine() {
        use std::os::unix::ffi::OsStrExt;
        let path = Path::new(OsStr::from_bytes(b"\xff")).join("a.md");
        assert_eq!(Log::key(&path).unwrap(), "a.md");
    }

    #[test]
    #[ignore = "prints error messages for manual inspection"]
    fn print_error_messages() {
        for fm in [
            "tags: []\n",
            "title: Foo\nlog:\n  status: beaten\n",
            "title: Foo\nlog:\n  status: played\n  rating: 6\n",
            "title: Foo\nlog:\n  rating: 0\n",
            "title: Foo\nlog:\n  started: yesterday\n",
        ] {
            println!("---- {:?}\n{}\n", fm, parse_err(fm).1);
        }
    }

    #[test]
    fn splits_frontmatter_and_body() {
        let (frontmatter, body) =
            split_frontmatter("---\ntitle: Foo\n---\n\nReview text\n").unwrap();
        assert_eq!(frontmatter, "title: Foo\n");
        assert_eq!(body, "Review text\n");
    }

    #[test]
    fn handles_crlf() {
        let (frontmatter, body) = split_frontmatter("---\r\ntitle: Foo\r\n---\r\nBody").unwrap();
        assert_eq!(frontmatter, "title: Foo\r\n");
        assert_eq!(body, "Body");
    }

    #[test]
    fn rejects_missing_opening_fence() {
        let error = split_frontmatter("title: Foo\n---\n").unwrap_err();
        assert_eq!(error, LogError::MissingFrontmatter);
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        let error = split_frontmatter("---\ntitle: Foo\n").unwrap_err();
        assert_eq!(error, LogError::UnclosedFrontmatter);
    }
}
