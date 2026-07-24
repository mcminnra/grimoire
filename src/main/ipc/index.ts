import { registerAppIpc } from './app'

// Central IPC registration — called once on app ready. Register each new
// domain's handlers here as the surface grows (games, config, …).
export function registerIpc(): void {
  registerAppIpc()
}
