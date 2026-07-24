import { House, ChartNoAxesColumn } from '@lucide/svelte'

// NOTE: single source of truth for the app's top-level views
export type View = 'home' | 'stats'

export interface NavItem {
  id: View
  label: string
  icon: typeof House
}

export const NAV_ITEMS: NavItem[] = [
  { id: 'home', label: 'Home', icon: House },
  { id: 'stats', label: 'Stats', icon: ChartNoAxesColumn }
]
