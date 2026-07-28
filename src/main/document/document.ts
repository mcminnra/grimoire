import { readdir, readFile } from 'node:fs/promises'
import { homedir } from 'node:os'
import { extname, join } from 'node:path'
import { parseDocument } from 'yaml'
import type { RawDocument, Collection } from '../../shared/types'

const FRONTMATTER = /^---[ \t]*\r?\n([\s\S]*?)\r?\n---[ \t]*\r?(?:\n|$)/

// TODO: Replace with real config reading
// HACK: node doesn't expand the shell tilde; drops out once config reading is real
const logDirectory = '~/tank/sync/logs/games'.replace(/^~/, homedir())

async function getRawDocuments(): Promise<RawDocument[]> {
  const entries = await readdir(logDirectory, { withFileTypes: true })
  const rawDocuments: RawDocument[] = []

  for (const entry of entries) {
    if (!entry.isFile()) continue
    if (extname(entry.name).toLowerCase() !== '.md') continue

    // Read file
    const filename = entry.name
    const filepath = join(entry.parentPath, filename)
    const contents = await readFile(filepath, 'utf8')

    // Parse yaml
    let frontmatter_raw = ''
    let body = ''
    const match = FRONTMATTER.exec(contents)

    if (!match) {
      // no frontmatter, all body
      frontmatter_raw = ''
      body = contents
    } else {
      frontmatter_raw = match[1]
      body = contents.slice(match[0].length)
    }

    const frontmatter = (parseDocument(frontmatter_raw).toJS() ?? {}) as Record<string, unknown>

    // TODO Capture yaml parsing issues - or maybe silently fail here and let GameEntry parsing catch it

    // Create doc
    const rawDocument: RawDocument = {
      filepath,
      filename,
      frontmatter,
      body
    }
    rawDocuments.push(rawDocument)
  }

  return rawDocuments
}

export async function getCollection(): Promise<Collection> {
  const rawDocuments = await getRawDocuments()
  console.log(rawDocuments)
  const collection: Collection = {}

  // TODO: parse rawDocuments

  return collection
}
