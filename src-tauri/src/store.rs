use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::sync::{Mutex, MutexGuard};

use crate::parse::log::Log;
use crate::parse::parse::load_logs;

pub type AppState = Mutex<LogStore>;

/// Locks the store.
pub fn lock(state: &AppState) -> io::Result<MutexGuard<'_, LogStore>> {
    state.lock().map_err(|e| io::Error::other(e.to_string()))
}

#[derive(Debug, Default)]
pub struct LogStore {
    logs: Option<BTreeMap<String, Log>>,
}

impl LogStore {
    pub fn logs(&mut self) -> io::Result<&mut BTreeMap<String, Log>> {
        let logs = match self.logs.take() {
            Some(logs) => logs,
            None => load_logs()?
                .into_iter()
                .map(|log| (log.file_name().to_owned(), log))
                .collect(),
        };
        Ok(self.logs.insert(logs))
    }

    pub fn get(&mut self, file_name: &str) -> io::Result<Option<&Log>> {
        Ok(self.logs()?.get(file_name))
    }

    pub fn save(&mut self, file_name: &str, source: String) -> io::Result<&Log> {
        let log = self.logs()?.get_mut(file_name).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, format!("no log named {file_name}"))
        })?;

        // TODO(agent): write to a temp file and rename so a crash mid-write can't truncate the log
        fs::write(log.path(), &source)
            .map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", log.path().display())))?;

        *log = Log::parse(log.path().to_owned(), source)?;
        Ok(log)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::error::LogError;
    use std::path::PathBuf;

    /// A store preloaded with one log backed by a fresh file under the OS temp dir.
    fn store_with(file_name: &str, source: &str) -> (LogStore, PathBuf) {
        let dir = std::env::temp_dir().join(format!("grimoire-store-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(file_name);
        fs::write(&path, source).unwrap();

        let log = Log::parse(path.clone(), source.into()).unwrap();
        let logs = BTreeMap::from([(file_name.to_string(), log)]);
        (LogStore { logs: Some(logs) }, path)
    }

    #[test]
    fn save_writes_and_reparses() {
        let (mut store, path) = store_with("save.md", "---\ntitle: Old\n---\n");
        let source = "---\ntitle: New\n---\nbody";

        let log = store.save("save.md", source.into()).unwrap();
        assert_eq!(log.metadata.as_ref().unwrap().title, "New");
        assert_eq!(fs::read_to_string(&path).unwrap(), source);
        assert_eq!(
            store.get("save.md").unwrap().unwrap().body.as_deref(),
            Some("body")
        );
    }

    #[test]
    fn save_with_invalid_frontmatter_still_writes() {
        let (mut store, path) = store_with("invalid.md", "---\ntitle: Old\n---\n");

        let log = store.save("invalid.md", "no fence".into()).unwrap();
        assert_eq!(log.error, Some(LogError::MissingFrontmatter));
        assert_eq!(fs::read_to_string(&path).unwrap(), "no fence");
    }

    #[test]
    fn save_unknown_file_name_is_not_found() {
        let (mut store, _) = store_with("known.md", "---\ntitle: Foo\n---\n");
        let error = store.save("unknown.md", String::new()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }
}
