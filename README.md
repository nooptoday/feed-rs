## Description

Node.js bindings for [feed_rs](https://github.com/feed-rs/feed-rs). 

## Installation

`npm install @nooptoday/feed-rs`

## Usage 

```typescript
import { parse } from '@nooptoday/feed-rs'

const response = await fetch("https://nooptoday.com/rss")
const rss = await response.text()
const feed = parse(rss, "https://nooptoday.com")

console.log(feed) // Feed Object
```
---
Models are simplified as follows:

- struct Person, replaced with Person.name property as String
- struct Link, replaced with Link.href property as String
- timestamps replaced with u32 ( epoch, milliseconds )
- struct Category, replaced with Category.term as String
- struct Generator, replaced with Generator.content as String
- and such...

## Benchmark