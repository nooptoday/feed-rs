import { readFileSync } from 'fs'

import b from 'benny'
import { XMLParser } from 'fast-xml-parser'
import Parser from 'rss-parser'

import { parse } from '../index'

async function run() {
  const feed = readFileSync('feed.xml', 'utf-8')
  const xmlParser = new XMLParser()
  const rssParser = new Parser()

  await b.suite(
    'Feed parsing',

    b.add('FeedRS', () => {
      parse(feed)
    }),

    b.add('RssParser', async () => {
      await rssParser.parseString(feed)
    }),

    b.add('FastXMLParser', () => {
      xmlParser.parse(feed)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
