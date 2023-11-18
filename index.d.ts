/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/**
 * Combined model for a syndication feed (i.e. RSS1, RSS 2, Atom, JSON Feed)
 *
 * The model is based on the Atom standard as a start with RSS1+2 mapped on to it e.g.
 * * Atom
 *     * Feed -> Feed
 *     * Entry -> Entry
 * * RSS 1 + 2
 *     * Channel -> Feed
 *     * Item -> Entry
 *
 * [Atom spec]: http://www.atomenabled.org/developers/syndication/
 * [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html
 * [RSS 1 spec]: https://validator.w3.org/feed/docs/rss1.html
 * [MediaRSS spec]: https://www.rssboard.org/media-rss
 * [iTunes podcast spec]: https://help.apple.com/itc/podcasts_connect/#/itcb54353390
 * [iTunes podcast guide]: https://www.feedforall.com/itune-tutorial-tags.htm
 *
 * Certain elements are not mapped given their limited utility:
 *   * RSS 2:
 *     * channel - docs (pointer to the spec), cloud (for callbacks), textInput (text box e.g. for search)
 *     * item - comments (link to comments on the article), source (pointer to the channel, but our data model links items to a channel)
 *   * RSS 1:
 *     * channel - rdf:about attribute (pointer to feed), textinput (text box e.g. for search)
 */
export interface Feed {
  /** Type of this feed (e.g. RSS2, Atom etc) */
  feedType: FeedType
  /**
   * A unique identifier for this feed
   * * Atom (required): Identifies the feed using a universally unique and permanent URI.
   * * RSS doesn't require an ID so it is initialised to the hash of the first link or a UUID if not found
   */
  id: string
  /**
   * The title of the feed
   * * Atom (required): Contains a human readable title for the feed. Often the same as the title of the associated website. This value should not be blank.
   * * RSS 1 + 2 (required) "title": The name of the channel. It's how people refer to your service.
   * * JSON Feed: is the name of the feed
   */
  title?: Text
  /**
   * The time at which the feed was last modified. If not provided in the source, or invalid, it is `None`.
   * * Atom (required): Indicates the last time the feed was modified in a significant way.
   * * RSS 2 (optional) "lastBuildDate": The last time the content of the channel changed.
   */
  updated?: number
  /**
   * Atom (recommended): Collection of authors defined at the feed level.
   * JSON Feed: specifies the feed author.
   */
  authors: Array<Person>
  /**
   * Description of the feed
   * * Atom (optional): Contains a human-readable description or subtitle for the feed (from <subtitle>).
   * * RSS 1 + 2 (required): Phrase or sentence describing the channel.
   * * JSON Feed: description of the feed
   */
  description?: Text
  /**
   * Links to related pages
   * * Atom (recommended): Identifies a related Web page.
   * * RSS 1 + 2 (required): The URL to the HTML website corresponding to the channel.
   * * JSON Feed: the homepage and feed URLs
   */
  links: Array<Link>
  /**
   * Structured classification of the feed
   * * Atom (optional): Specifies a category that the feed belongs to. A feed may have multiple category elements.
   * * RSS 2 (optional) "category": Specify one or more categories that the channel belongs to.
   */
  categories: Array<Category>
  /**
   * People who have contributed to the feed
   * * Atom (optional): Names one contributor to the feed. A feed may have multiple contributor elements.
   * * RSS 2 (optional) "managingEditor": Email address for person responsible for editorial content.
   * * RSS 2 (optional) "webMaster": Email address for person responsible for technical issues relating to channel.
   */
  contributors: Array<Person>
  /**
   * Information on the software used to build the feed
   * * Atom (optional): Identifies the software used to generate the feed, for debugging and other purposes.
   * * RSS 2 (optional): A string indicating the program used to generate the channel.
   */
  generator?: Generator
  /**
   * A small icon
   * * Atom (optional): Identifies a small image which provides iconic visual identification for the feed.
   * * JSON Feed: is the URL of an image for the feed suitable to be used in a source list.
   */
  icon?: Image
  /** RSS 2 (optional): The language the channel is written in. */
  language?: string
  /**
   * An image used to visually identify the feed
   * * Atom (optional): Identifies a larger image which provides visual identification for the feed.
   * * RSS 1 + 2 (optional) "image": Specifies a GIF, JPEG or PNG image that can be displayed with the channel.
   * * JSON Feed: is the URL of an image for the feed suitable to be used in a timeline
   */
  logo?: Image
  /** RSS 2 (optional): The publication date for the content in the channel. */
  published?: number
  /**
   * Rating for the content
   * * Populated from the media or itunes namespaces
   */
  rating?: MediaRating
  /**
   * Rights restricting content within the feed
   * * Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
   * * RSS 2 (optional) "copyright": Copyright notice for content in the channel.
   */
  rights?: Text
  /** RSS 2 (optional): It's a number of minutes that indicates how long a channel can be cached before refreshing from the source. */
  ttl?: number
  /**
   * The individual items within the feed
   * * Atom (optional): Individual entries within the feed (e.g. a blog post)
   * * RSS 1+2 (optional): Individual items within the channel.
   */
  entries: Array<Entry>
}
/** Type of a feed (RSS, Atom etc) */
export const enum FeedType {
  Atom = 'Atom',
  JSON = 'JSON',
  RSS0 = 'RSS0',
  RSS1 = 'RSS1',
  RSS2 = 'RSS2',
}
/** An item within a feed */
export interface Entry {
  /**
   * A unique identifier for this item with a feed. If not supplied it is initialised to a hash of the first link or a UUID if not available.
   * * Atom (required): Identifies the entry using a universally unique and permanent URI.
   * * RSS 2 (optional) "guid": A string that uniquely identifies the item.
   * * RSS 1: does not specify a unique ID as a separate item, but does suggest the URI should be "the same as the link" so we use a hash of the link if found
   * * JSON Feed: is unique for that item for that feed over time.
   */
  id: string
  /**
   * Title of this item within the feed
   * * Atom, RSS 1(required): Contains a human readable title for the entry.
   * * RSS 2 (optional): The title of the item.
   * * JSON Feed: The title of the item.
   */
  title?: Text
  /**
   * Time at which this item was last modified. If not provided in the source, or invalid, it is `None`.
   * * Atom (required): Indicates the last time the entry was modified in a significant way.
   * * RSS doesn't specify this field.
   * * JSON Feed: the last modification date of this item
   */
  updated?: number
  /**
   * Authors of this item
   * * Atom (recommended): Collection of authors defined at the entry level.
   * * RSS 2 (optional): Email address of the author of the item.
   * * JSON Feed: the author of the item
   */
  authors: Array<Person>
  /**
   * The content of the item
   * * Atom (recommended): Contains or links to the complete content of the entry.
   * * RSS 2 (optional) "content:encoded": The HTML form of the content
   * * JSON Feed: the html content of the item, or the text content if no html is specified
   */
  content?: Content
  /**
   * Links associated with this item
   * * Atom (recommended): Identifies a related Web page.
   * * RSS 2 (optional): The URL of the item.
   * * RSS 1 (required): The item's URL.
   * * JSON Feed: the url and external URL for the item is the first items, then each subsequent attachment
   */
  links: Array<Link>
  /**
   * A short summary of the item
   * * Atom (recommended): Conveys a short summary, abstract, or excerpt of the entry.
   * * RSS 1 (optional): Populated from the RSS namespace 'description' field, or if not present, the Dublin Core namespace 'description' field.
   * * RSS 2 (optional): Populated from the RSS namespace 'description' field.
   * * JSON Feed: the summary for the item, or the text content if no summary is provided and both text and html content are specified
   *
   * Warning: Some feeds (especially RSS) use significant whitespace in this field even in cases where it should be considered HTML. Consider rendering this field in a way that preserves whitespace-based formatting such as a double-newline to separate paragraphs.
   */
  summary?: Text
  /**
   * Structured classification of the item
   * * Atom (optional): Specifies a category that the entry belongs to. A feed may have multiple category elements.
   * * RSS 2 (optional): Includes the item in one or more categories.
   * * JSON Feed: the supplied item tags
   */
  categories: Array<Category>
  /** Atom (optional): Names one contributor to the entry. A feed may have multiple contributor elements. */
  contributors: Array<Person>
  /**
   * Time at which this item was first published
   * * Atom (optional): Contains the time of the initial creation or first availability of the entry.
   * * RSS 2 (optional) "pubDate": Indicates when the item was published.
   * * JSON Feed: the date at which the item was published
   */
  published?: number
  /** Atom (optional): If an entry is copied from one feed into another feed, then this contains the source feed metadata. */
  source?: string
  /** Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed. */
  rights?: Text
  /**
   * Extension for MediaRSS - https://www.rssboard.org/media-rss
   * A MediaObject will be created in two cases:
   * 1) each "media:group" element encountered in the feed
   * 2) a default for any other "media:*" elements found at the item level
   * See the Atom tests for youtube and newscred for examples
   */
  media: Array<MediaObject>
}
/**
 * Represents the category of a feed or entry
 *
 * [Atom spec]: http://www.atomenabled.org/developers/syndication/#category
 * [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html#ltcategorygtSubelementOfLtitemgt
 */
export interface Category {
  /**
   * The category as a human readable string
   * * Atom (required): Identifies the category.
   * * RSS 2: The value of the element is a forward-slash-separated string that identifies a hierarchic location in the indicated taxonomy. Processors may establish conventions for the interpretation of categories.
   * * JSON Feed: the value of the tag
   */
  term: string
  /** Atom (optional): Identifies the categorization scheme via a URI. */
  scheme?: string
  /** Atom (optional): Provides a human-readable label for display. */
  label?: string
}
/**
 * Content, or link to the content, for a given entry.
 *
 * [Atom spec]: http://www.atomenabled.org/developers/syndication/#contentElement
 * [RSS 2.0]: https://validator.w3.org/feed/docs/rss2.html#ltenclosuregtSubelementOfLtitemgt
 */
export interface Content {
  /**
   * Atom
   * * If the type attribute ends in +xml or /xml, then an xml document of this type is contained inline.
   * * If the type attribute starts with text, then an escaped document of this type is contained inline.
   * * Otherwise a base64 encoded document of the indicated media type is contained inline.
   */
  body?: string
  /**
   * Type of content
   * * Atom: The type attribute is either text, html, xhtml, in which case the content element is defined identically to other text constructs.
   * * RSS 2: Type says what its type is, a standard MIME type
   */
  contentType: string
  /** RSS 2.0: Length of the content in bytes */
  length?: number
  /**
   * Source of the content
   * * Atom: If the src attribute is present, it represents the URI of where the content can be found. The type attribute, if present, is the media type of the content.
   * * RSS 2.0: where the enclosure is located
   */
  src?: Link
}
/**
 * Information on the tools used to generate the feed
 *
 * Atom: Identifies the software used to generate the feed, for debugging and other purposes.
 */
export interface Generator {
  /**
   * Atom: Additional data
   * RSS 2: A string indicating the program used to generate the channel.
   */
  content: string
  /** Atom: Link to the tool */
  uri?: string
  /** Atom: Tool version */
  version?: string
}
/**
 * Represents a a link to an image.
 *
 * [Atom spec]:  http://www.atomenabled.org/developers/syndication/#optionalFeedElements
 * [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html#ltimagegtSubelementOfLtchannelgt
 * [RSS 1 spec]: https://validator.w3.org/feed/docs/rss1.html#s5.4
 */
export interface Image {
  /**
   * Link to the image
   * * Atom: The URL to an image or logo
   * * RSS 1 + 2: the URL of a GIF, JPEG or PNG image that represents the channel.
   */
  uri: string
  /** RSS 1 + 2: describes the image, it's used in the ALT attribute of the HTML <img> tag when the channel is rendered in HTML. */
  title?: string
  /** RSS 1 + 2: the URL of the site, when the channel is rendered, the image is a link to the site. */
  link?: Link
  /** RSS 2 (optional): width of the image */
  width?: number
  /** RSS 2 (optional): height of the image */
  height?: number
  /** RSS 2 (optional): contains text that is included in the TITLE attribute of the link formed around the image in the HTML rendering. */
  description?: string
}
/**
 * Represents a link to an associated resource for the feed or entry.
 *
 * [Atom spec]: http://www.atomenabled.org/developers/syndication/#link
 */
export interface Link {
  /**
   * Link to additional content
   * * Atom: The URI of the referenced resource (typically a Web page).
   * * RSS 2: The URL to the HTML website corresponding to the channel or item.
   * * JSON Feed: the URI to the attachment, feed etc
   */
  href: string
  /** A single link relationship type. */
  rel?: string
  /** Indicates the media type of the resource. */
  mediaType?: string
  /** Indicates the language of the referenced resource. */
  hrefLang?: string
  /** Human readable information about the link, typically for display purposes. */
  title?: string
  /** The length of the resource, in bytes. */
  length?: number
}
/**
 * The top-level representation of a media object
 * i.e. combines "media:*" elements from the RSS Media spec such as those under a media:group
 */
export interface MediaObject {
  /** Title of the object (from the media:title element) */
  title?: Text
  /** Collection of the media content elements */
  content: Array<MediaContent>
  /** Duration of the object */
  duration?: number
  /** Representative images for the object (from media:thumbnail elements) */
  thumbnails: Array<MediaThumbnail>
  /** A text transcript, closed captioning or lyrics of the media content. */
  texts: Array<MediaText>
  /** Short description of the media object (from the media:description element) */
  description?: Text
  /** Community info (from the media:community element) */
  community?: MediaCommunity
  /** Credits */
  credits: Array<MediaCredit>
}
/** Represents a "media:community" item from the RSS Media spec */
export interface MediaCommunity {
  /** Star rating */
  starsAvg?: number
  starsCount?: number
  starsMin?: number
  starsMax?: number
  /** Statistics on engagement */
  statsViews?: number
  statsFavorites?: number
}
/** Represents a "media:content" item from the RSS Media spec */
export interface MediaContent {
  /** The direct URL */
  url?: string
  /** Standard MIME type */
  contentType?: string
  /** Height and width */
  height?: number
  width?: number
  /** Duration the media plays */
  duration?: number
  /** Size of media in bytes */
  size?: number
  /** Rating */
  rating?: MediaRating
}
/** Represents a "media:credit" item from the RSS Media spec */
export interface MediaCredit {
  /** The entity being credited */
  entity: string
}
/** Rating of the feed, item or media within the content */
export interface MediaRating {
  urn: string
  value: string
}
/** Represents a "media:text" item from the RSS Media spec */
export interface MediaText {
  /** The text */
  text: Text
  /** The start time offset that the text starts being relevant to the media object. */
  startTime?: number
  /** The end time that the text is relevant. If this attribute is not provided, and a start time is used, it is expected that the end time is either the end of the clip or the start of the next <media:text> element. */
  endTime?: number
}
/** Represents a "media:thumbnail" item from the RSS Media spec */
export interface MediaThumbnail {
  /** The thumbnail image */
  image: Image
  /** The time this thumbnail represents */
  time?: number
}
/**
 * Represents an author, contributor etc.
 *
 * [Atom spec]: http://www.atomenabled.org/developers/syndication/#person
 */
export interface Person {
  /**
   * Atom: human-readable name for the person.
   * JSON Feed: human-readable name for the person.
   */
  name: string
  /**
   * Atom: home page for the person.
   * JSON Feed: link to media (Twitter etc) for the person
   */
  uri?: string
  /** Atom: An email address for the person. */
  email?: string
}
/** Textual content, or link to the content, for a given entry. */
export interface Text {
  contentType: string
  src?: string
  content: string
}
/**
 * @param feedString Supported feeds are RSS0, RSS1, RSS2, Atom and JSON
 * @param feedSource Optional source of the content, used to resolve relative URLs in XML based
 * feeds
 * @example
 * import { parse } from '@nooptoday/feed-rs'
 *
 * const response = await fetch('https://nooptoday.com/rss')
 * const rss = await response.text()
 * const feed = parse(rss, 'https://nooptoday.com')
 * @description
 * Parses given feed string into a common Feed format defined in feed-rs.
 * @exception Error no root element
 * @exception Error unsupported content type {mime}
 * @exception Error missing content element {element}
 * @exception Error unable to read feed: {reason}
 * @exception Error unable to parse JSON: {reason}
 * @exception Error unsupported version: {version}
 * @exception Error unable to parse XML: {reason}
 */
export function parse(feedString: string, feedSource?: string | undefined | null): Feed
