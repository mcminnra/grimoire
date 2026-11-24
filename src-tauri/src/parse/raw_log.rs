use std::fs;
use std::path::Path;

use super::error::{FailedLog, LogLoadError};

const DELIMITER: &str = "---";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawLog {
    pub file_name: String, // NOTE: This is a log's unique key other systems will pin to. e.g. `007-first-light.md`
    /// Full file contents, kept so a later failure can hand it to the user to fix.
    pub source: String,
    pub frontmatter: String,
    pub body: String,
}

impl RawLog {
    /// Reads the file at `path` and splits it with [`RawLog::parse`].
    ///
    /// # Errors
    ///
    /// Returns a [`FailedLog`] when the file can't be read or is malformed.
    pub fn read(path: &Path) -> Result<Self, FailedLog> {
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();

        match fs::read_to_string(path) {
            Ok(source) => Self::parse(file_name, source),
            Err(e) => Err(FailedLog {
                file_name,
                source: None,
                error: LogLoadError::Io(e.to_string()),
            }),
        }
    }

    /// Splits `source` of the file `file_name` on the leading `---` frontmatter fences.
    ///
    /// The first line must be `---`. The frontmatter runs until the next line that is
    /// exactly `---`. Everything after that line is the body.
    ///
    /// # Errors
    ///
    /// Returns a [`FailedLog`] when the opening or closing fence is missing.
    pub fn parse(file_name: String, source: String) -> Result<Self, FailedLog> {
        match split(&source) {
            Ok((frontmatter, body)) => Ok(Self {
                file_name,
                frontmatter,
                body,
                source,
            }),
            Err(error) => Err(FailedLog {
                file_name,
                source: Some(source),
                error,
            }),
        }
    }
}

/// Splits `source` into owned `(frontmatter, body)` strings.
fn split(source: &str) -> Result<(String, String), LogLoadError> {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let mut lines = source.split_inclusive('\n');

    let opening = lines.next().unwrap_or_default();
    if opening.trim_end() != DELIMITER {
        return Err(LogLoadError::MissingFrontmatter);
    }

    let mut frontmatter = String::new();
    for line in lines.by_ref() {
        if line.trim_end() == DELIMITER {
            let body = lines.collect::<String>().trim_start().to_string();
            return Ok((frontmatter, body));
        }
        frontmatter.push_str(line);
    }

    Err(LogLoadError::UnclosedFrontmatter)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Result<RawLog, FailedLog> {
        RawLog::parse("a.md".into(), source.into())
    }

    #[test]
    fn splits_frontmatter_and_body() {
        let source = "---\ntitle: Foo\n---\n\nReview text\n";
        let raw = parse(source).unwrap();
        assert_eq!(raw.file_name, "a.md");
        assert_eq!(raw.frontmatter, "title: Foo\n");
        assert_eq!(raw.body, "Review text\n");
        assert_eq!(raw.source, source);
    }

    #[test]
    fn handles_crlf() {
        let raw = parse("---\r\ntitle: Foo\r\n---\r\nBody").unwrap();
        assert_eq!(raw.frontmatter, "title: Foo\r\n");
        assert_eq!(raw.body, "Body");
    }

    #[test]
    fn rejects_missing_opening_fence() {
        let failed = parse("title: Foo\n---\n").unwrap_err();
        assert_eq!(failed.file_name, "a.md");
        assert_eq!(failed.error, LogLoadError::MissingFrontmatter);
        assert_eq!(failed.source.as_deref(), Some("title: Foo\n---\n"));
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        let failed = parse("---\ntitle: Foo\n").unwrap_err();
        assert_eq!(failed.error, LogLoadError::UnclosedFrontmatter);
    }
}
