# Grimoire

## What this is

Grimoire is a personal game tracker and journaling tool. It helps the user maintain a collection of plain-text files (one per game) for games played and games to play. The design treats the collection as a private, durable, portable knowledge artifact — tools are interfaces over the files, not the source of truth.

## How to run

This is an Electron + Vite + Svelte + TypeScript app. If a `flake.nix` is present, enter the dev shell first (`nix develop`) so you get the pinned toolchain (Node LTS). Then use the npm scripts.

Scripts are the source of truth — check `package.json` `scripts` for the current command surface before assuming. As of now:

- dev (run in watch mode): `npm run dev`
- build: `npm run build` (runs `typecheck` then `electron-vite build`)
- preview a build: `npm run start`
- typecheck: `npm run typecheck` (node + svelte-check)
- lint: `npm run lint`
- format code: `npm run format`
- platform packages: `npm run build:mac` / `build:win` / `build:linux`
