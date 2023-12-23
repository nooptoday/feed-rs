mod models;

use feed_rs::parser;
use napi::{Env, Error};
use napi_derive::napi;
use std::option::Option;

/// @param feedString Supported feeds are RSS0, RSS1, RSS2, Atom and JSON
/// @param feedSource Optional source of the content, used to resolve relative URLs in XML based
/// feeds
/// @example
/// import { parse } from '@nooptoday/feed-rs'
///
/// const response = await fetch('https://nooptoday.com/rss')
/// const rss = await response.text()
/// const feed = parse(rss, 'https://nooptoday.com')
/// @description
/// Parses given feed string into a common Feed format defined in feed-rs.
/// @exception Error no root element
/// @exception Error unsupported content type {mime}
/// @exception Error missing content element {element}
/// @exception Error unable to read feed: {reason}
/// @exception Error unable to parse JSON: {reason}
/// @exception Error unsupported version: {version}
/// @exception Error unable to parse XML: {reason}
#[napi]
pub fn parse(
  env: Env,
  feed_string: String,
  feed_source: Option<String>,
) -> Result<models::Feed, Error> {
  let result = parser::parse_with_uri(
    feed_string.as_bytes(),
    feed_source.as_ref().map(|source| source.as_str()),
  );

  match result {
    Ok(feed) => Ok(models::Feed::from(env, feed)),
    Err(err) => Err(Error::from_reason(err.to_string())),
  }
}
