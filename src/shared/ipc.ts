// Shared IPC contract: the single source of truth for channel names (and, as
// the app grows, request payload/return types), imported by BOTH the main
// process (handlers) and the preload (senders) so the two can never drift.

export const channels = {
  // Fire-and-forget commands (renderer → main, no response): ipcRenderer.send / ipcMain.on
  app: {
    quit: 'app:quit'
  }

  // Request/response (renderer → main → result): ipcRenderer.invoke / ipcMain.handle
  // Add domains here as the collection API grows, e.g.:
  // games: {
  //   list: 'games:list',
  //   read: 'games:read',
  //   write: 'games:write'
  // }
} as const

// Shared payload + return types for request channels live here so both sides
// agree on one shape, e.g.:
// export interface GameSummary {
//   id: string
//   title: string
//   status: string
// }
