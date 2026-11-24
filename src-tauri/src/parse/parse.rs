use std::fs;
use std::io;
use std::path::PathBuf;

use serde::Serialize;

use super::error::{FailedLog, LogLoadError};
use super::log::Log;
use super::raw_log::RawLog;

// TODO: This should exist in settings once that feature exists
const GAME_LOG_DIR: &str = "tank/sync/logs/games";

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLoadStatus {
    pub logs: Vec<Log>,
    pub failed: Vec<FailedLog>,
}

/// Load game logs
///
/// TODO: eventually we'll want this function to read settings, but for now it's hardcoded path
///
/// # Errors
/// - Fails if the home directory is unknown or the log directory cannot be listed.
/// - Fails if the log directory doesn't exist or isn't a directory.
pub fn load_game_logs() -> io::Result<LogLoadStatus> {
    // Check home
    let home = std::env::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "home directory not found"))?;

    // Ensure log dir
    let dir = home.join(GAME_LOG_DIR);
    match fs::metadata(&dir) {
        Ok(meta) if meta.is_dir() => {}
        Ok(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("log path is not a directory: {}", dir.display()),
            ));
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("log directory not found: {}", dir.display()),
            ));
        }
        Err(e) => return Err(io::Error::new(e.kind(), format!("{}: {e}", dir.display()))),
    }

    // Collect `.md` files
    let mut paths: Vec<PathBuf> = Vec::new();
    let with_path = |e: io::Error| io::Error::new(e.kind(), format!("{}: {e}", dir.display()));
    for entry in fs::read_dir(&dir).map_err(with_path)? {
        let path = entry.map_err(with_path)?.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            paths.push(path);
        }
    }
    paths.sort();

    // Parse logs
    let mut status = LogLoadStatus::default();
    for path in &paths {
        match RawLog::read(path).and_then(parse_log) {
            Ok(log) => status.logs.push(log),
            Err(failed) => status.failed.push(failed),
        }
    }
    Ok(status)
}

/// Parse a log
///
/// # Errors
///
/// Returns a [`FailedLog`] carrying the full source when the frontmatter is invalid YAML,
/// doesn't match the [`Log`] shape, or fails a value check like [`Rating`](super::log::Rating).
pub fn parse_log(raw: RawLog) -> Result<Log, FailedLog> {
    // NOTE: re-adding the `---` fence keeps YAML error line numbers equal to file line numbers
    let yaml = format!("---\n{}", raw.frontmatter);

    match serde_saphyr::from_str::<Log>(&yaml) {
        Ok(mut log) => {
            log.file_name = raw.file_name;
            log.body = raw.body;
            Ok(log)
        }
        Err(e) => Err(FailedLog {
            file_name: raw.file_name,
            source: Some(raw.source),
            error: LogLoadError::Parse(e.to_string()),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::log::{Rating, Status};

    fn parse(frontmatter: &str) -> Result<Log, FailedLog> {
        let source = format!("---\n{frontmatter}---\nbody");
        RawLog::parse("test.md".into(), source).and_then(parse_log)
    }

    #[test]
    fn parses_full_log() {
        let log = parse(
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
        )
        .unwrap();

        assert_eq!(log.file_name, "test.md");
        assert_eq!(log.title, "World of Warcraft: Midnight");
        assert_eq!(
            log.tags.as_deref(),
            Some(&["mmo".to_string(), "blizzard".to_string()][..])
        );
        assert_eq!(log.details.status, Some(Status::CompletedPlus));
        assert_eq!(log.details.rating.map(Rating::get), Some(4));
        assert_eq!(log.details.started, Some(jiff::civil::date(2026, 3, 2)));
        assert_eq!(log.details.hours_played, Some(120.0));
        assert_eq!(log.details.revisit, Some(true));
        assert_eq!(log.provider_ids.steam, Some(12345));
        assert_eq!(log.body, "body");
    }

    #[test]
    fn nulls_and_unknown_keys_are_fine() {
        let log = parse("title: Foo\ntags: null\ncover: null\nlog:\n  rating: null\n").unwrap();
        assert_eq!(log.tags, None);
        assert_eq!(log.details.rating, None);
    }

    #[test]
    fn missing_title_is_a_parse_error() {
        let failed = parse("tags: []\n").unwrap_err();
        assert!(
            matches!(failed.error, LogLoadError::Parse(_)),
            "{}",
            failed.error
        );
        assert_eq!(failed.source.as_deref(), Some("---\ntags: []\n---\nbody"));
    }

    #[test]
    fn unknown_status_is_a_parse_error() {
        let failed = parse("title: Foo\nlog:\n  status: beaten\n").unwrap_err();
        assert!(
            matches!(failed.error, LogLoadError::Parse(_)),
            "{}",
            failed.error
        );
    }

    #[test]
    fn rating_out_of_range_is_a_parse_error_with_location() {
        for rating in ["0", "6", "-1", "300"] {
            let failed = parse(&format!("title: Foo\nlog:\n  rating: {rating}\n")).unwrap_err();
            let LogLoadError::Parse(message) = &failed.error else {
                panic!("expected parse error, got {:?}", failed.error);
            };
            assert!(message.contains("line 4"), "{message}");
        }
    }

    #[test]
    fn invalid_yaml_is_a_parse_error() {
        let failed = parse("title: [unclosed\n").unwrap_err();
        assert!(
            matches!(failed.error, LogLoadError::Parse(_)),
            "{}",
            failed.error
        );
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
            println!("---- {:?}\n{}\n", fm, parse(fm).unwrap_err().error);
        }
    }
}
