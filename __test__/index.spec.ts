import { readFileSync } from 'fs'

import test from 'ava'

import { parse } from '../index'

test('Basic parse', (t) => {
  const feed = readFileSync('feed.xml', 'utf-8')
  const result = parse(feed, 'https://nooptoday.com')

  t.assert(result.updated instanceof Date)

  t.assert(result.title?.contentType === 'text/plain')
  t.assert(result.title?.content === 'Noop Today')

  t.assert(result.entries.length === 15)
  t.assert(result.generator?.content === 'Ghost 5.73')

  t.assert(Array.isArray(result.entries))
})
