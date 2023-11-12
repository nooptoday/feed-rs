import test from 'ava'

import { parse } from '../index'

test('sync function from native code', async (t) => {
  const feed = await getRssData('https://nooptoday.com/rss')
  const result = parse(feed, 'https://nooptoday.com')

  t.assert(Array.isArray(result.entries))
})

async function getRssData(url: string): Promise<string> {
  const response = await fetch(url)
  return response.text()
}
