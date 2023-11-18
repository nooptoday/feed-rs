# Fastest RSS parser in the NPM 🚀

Thanks to the power of Rust and the upstream repository authors, this package is the fastest RSS parser you can find.

## Description

Feed parser for Node.js. List of supported feeds

- RSS0
- RSS1
- RSS2
- Atom
- JSON

This package contains Node.js bindings from the Rust crate [feed_rs](https://github.com/feed-rs/feed-rs).

I've tried to be as much compliant as possible with the original package's models. The Rust crate also contains lots
of helpful comments, which I've also included in this package.

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

## Error Handling

As long as the input contains valid feed data, there shouldn't be any errors. In case something goes wrong, these
are all the possible errors you might encounter.

```
no root element
unsupported content type {mime}
missing content element {element}
unable to read feed: {reason}
unable to parse JSON: {reason}
unsupported version: {version}
unable to parse XML: {reason}
```

## Benchmark

Test results are obtained from a local M2 Air installation.

```
feed-rs          2367 ops/s, ±0.39%   | fastest
fast-xml-parser  1198 ops/s, ±0.26%   | 49.39% slower
rss-parser:      125 ops/s,  ±2.27%   | slowest, 94.72% slower
```
