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

## Benchmark

Test results are obtained from a local M2 Air installation.

```
feed-rs          2367 ops/s, ±0.39%   | fastest
fast-xml-parser  1198 ops/s, ±0.26%   | 49.39% slower
rss-parser:      125 ops/s,  ±2.27%   | slowest, 94.72% slower
```
