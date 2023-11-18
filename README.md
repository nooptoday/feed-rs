## Description

Node.js bindings for [feed_rs](https://github.com/feed-rs/feed-rs).

This package is the fastest RSS parser in NPM.

## Installation

`npm install @nooptoday/feed-rs`

## Usage

```typescript
import { parse } from '@nooptoday/feed-rs'

const response = await fetch('https://nooptoday.com/rss')
const rss = await response.text()
const feed = parse(rss, 'https://nooptoday.com')

console.log(feed) // Feed Object
```
---

Timestamps are converted to i64 fields

## Benchmark

```
feed-rs         1362 ops/s, ±0.25%    | fastest
rss-parser:     76 ops/s, ±0.16%      | slowest, 94.42% slower
fast-xml-parser 682 ops/s, ±0.32%     | 49.93% slower
```