import { getLog, getLogs, type LogView } from "$lib/log";

// NOTE: module-level state is safe here only because the app never server-renders (`ssr = false`)

/** Frontend mirror of backend log state. The backend stays the source of truth. */
class LogStore {
  /** Every log, valid and failed, in file name order. */
  logs = $state<LogView[]>([]);

  /**
   * Replaces every log with the backend's current state.
   *
   * @throws The backend's message if the log directory cannot be loaded.
   */
  async load(): Promise<void> {
    this.logs = await getLogs();
  }

  /**
   * Refetches one log after the backend reports a change to it.
   *
   * Replaces the log in place, inserts it in file name order if new, or drops it if the
   * backend no longer has it.
   *
   * @param fileName Key of the changed log.
   * @throws The backend's message if the log directory cannot be loaded.
   */
  async refresh(fileName: string): Promise<void> {
    const log = await getLog(fileName);
    const index = this.logs.findIndex((l) => l.fileName === fileName);

    if (log === null) {
      if (index !== -1) this.logs.splice(index, 1);
    } else if (index !== -1) {
      this.logs[index] = log;
    } else {
      // NOTE: plain `<` instead of localeCompare to stay close to the backend's BTreeMap byte order
      this.logs = [...this.logs, log].toSorted((a, b) =>
        a.fileName < b.fileName ? -1 : a.fileName > b.fileName ? 1 : 0,
      );
    }
  }
}

/** The app's single log store. */
export const logStore = new LogStore();
