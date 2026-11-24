use std::fs;
use std::io;
use std::path::PathBuf;

use super::log::Log;

// TODO: This should exist in settings once that feature exists
const GAME_LOG_DIR: &str = "tank/sync/logs/games";

/// Load game logs, one [`Log`] per `.md` file sorted by path. Per-file failures are on [`Log::error`].
///
/// TODO: eventually we'll want this function to read settings, but for now it's hardcoded path
///
/// # Errors
/// - Fails if the home directory is unknown or the log directory cannot be listed.
/// - Fails if the log directory doesn't exist or isn't a directory.
/// - Fails if any `.md` file name isn't a valid key, see [`Log::load`].
pub fn load_logs() -> io::Result<Vec<Log>> {
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
    paths.into_iter().map(Log::load).collect()
}
