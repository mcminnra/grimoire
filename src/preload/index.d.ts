import { ElectronAPI } from '@electron-toolkit/preload'

interface Api {
  app: {
    quit: () => void
  }
}

declare global {
  interface Window {
    electron: ElectronAPI
    api: Api
  }
}
