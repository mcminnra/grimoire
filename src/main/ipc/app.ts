import { app, ipcMain } from 'electron'
import { channels } from '../../shared/ipc'

// App lifecycle IPC — fire-and-forget commands from the renderer
export function registerAppIpc(): void {
  ipcMain.on(channels.app.quit, () => app.quit())
}
