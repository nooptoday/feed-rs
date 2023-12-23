use feed_rs::model;
use napi::{Env, JsDate};
use napi_derive::napi;
use num_traits::cast::FromPrimitive;

/// Combined model for a syndication feed (i.e. RSS1, RSS 2, Atom, JSON Feed)
///
/// The model is based on the Atom standard as a start with RSS1+2 mapped on to it e.g.
/// * Atom
///     * Feed -> Feed
///     * Entry -> Entry
/// * RSS 1 + 2
///     * Channel -> Feed
///     * Item -> Entry
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/
/// [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html
/// [RSS 1 spec]: https://validator.w3.org/feed/docs/rss1.html
/// [MediaRSS spec]: https://www.rssboard.org/media-rss
/// [iTunes podcast spec]: https://help.apple.com/itc/podcasts_connect/#/itcb54353390
/// [iTunes podcast guide]: https://www.feedforall.com/itune-tutorial-tags.htm
///
/// Certain elements are not mapped given their limited utility:
///   * RSS 2:
///     * channel - docs (pointer to the spec), cloud (for callbacks), textInput (text box e.g. for search)
///     * item - comments (link to comments on the article), source (pointer to the channel, but our data model links items to a channel)
///   * RSS 1:
///     * channel - rdf:about attribute (pointer to feed), textinput (text box e.g. for search)
// #[derive(Debug)]
#[napi(object)]
pub struct Feed {
  /// Type of this feed (e.g. RSS2, Atom etc)
  pub feed_type: FeedType,
  /// A unique identifier for this feed
  /// * Atom (required): Identifies the feed using a universally unique and permanent URI.
  /// * RSS doesn't require an ID so it is initialised to the hash of the first link or a UUID if not found
  pub id: String,
  /// The title of the feed
  /// * Atom (required): Contains a human readable title for the feed. Often the same as the title of the associated website. This value should not be blank.
  /// * RSS 1 + 2 (required) "title": The name of the channel. It's how people refer to your service.
  /// * JSON Feed: is the name of the feed
  pub title: Option<Text>,
  /// The time at which the feed was last modified. If not provided in the source, or invalid, it is `None`.
  /// * Atom (required): Indicates the last time the feed was modified in a significant way.
  /// * RSS 2 (optional) "lastBuildDate": The last time the content of the channel changed.
  pub updated: Option<JsDate>,

  /// Atom (recommended): Collection of authors defined at the feed level.
  /// JSON Feed: specifies the feed author.
  pub authors: Vec<Person>,
  /// Description of the feed
  /// * Atom (optional): Contains a human-readable description or subtitle for the feed (from <subtitle>).
  /// * RSS 1 + 2 (required): Phrase or sentence describing the channel.
  /// * JSON Feed: description of the feed
  pub description: Option<Text>,
  /// Links to related pages
  /// * Atom (recommended): Identifies a related Web page.
  /// * RSS 1 + 2 (required): The URL to the HTML website corresponding to the channel.
  /// * JSON Feed: the homepage and feed URLs
  pub links: Vec<Link>,

  /// Structured classification of the feed
  /// * Atom (optional): Specifies a category that the feed belongs to. A feed may have multiple category elements.
  /// * RSS 2 (optional) "category": Specify one or more categories that the channel belongs to.
  pub categories: Vec<Category>,
  /// People who have contributed to the feed
  /// * Atom (optional): Names one contributor to the feed. A feed may have multiple contributor elements.
  /// * RSS 2 (optional) "managingEditor": Email address for person responsible for editorial content.
  /// * RSS 2 (optional) "webMaster": Email address for person responsible for technical issues relating to channel.
  pub contributors: Vec<Person>,
  /// Information on the software used to build the feed
  /// * Atom (optional): Identifies the software used to generate the feed, for debugging and other purposes.
  /// * RSS 2 (optional): A string indicating the program used to generate the channel.
  pub generator: Option<Generator>,
  /// A small icon
  /// * Atom (optional): Identifies a small image which provides iconic visual identification for the feed.
  /// * JSON Feed: is the URL of an image for the feed suitable to be used in a source list.
  pub icon: Option<Image>,
  /// RSS 2 (optional): The language the channel is written in.
  pub language: Option<String>,
  /// An image used to visually identify the feed
  /// * Atom (optional): Identifies a larger image which provides visual identification for the feed.
  /// * RSS 1 + 2 (optional) "image": Specifies a GIF, JPEG or PNG image that can be displayed with the channel.
  /// * JSON Feed: is the URL of an image for the feed suitable to be used in a timeline
  pub logo: Option<Image>,
  /// RSS 2 (optional): The publication date for the content in the channel.
  pub published: Option<JsDate>,
  /// Rating for the content
  /// * Populated from the media or itunes namespaces
  pub rating: Option<MediaRating>,
  /// Rights restricting content within the feed
  /// * Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
  /// * RSS 2 (optional) "copyright": Copyright notice for content in the channel.
  pub rights: Option<Text>,
  /// RSS 2 (optional): It's a number of minutes that indicates how long a channel can be cached before refreshing from the source.
  pub ttl: Option<u32>,

  /// The individual items within the feed
  /// * Atom (optional): Individual entries within the feed (e.g. a blog post)
  /// * RSS 1+2 (optional): Individual items within the channel.
  pub entries: Vec<Entry>,
}

/// Type of a feed (RSS, Atom etc)
#[derive(Debug)]
#[napi(string_enum)]
pub enum FeedType {
  Atom,
  JSON,
  RSS0,
  RSS1,
  RSS2,
}

/// An item within a feed
// #[derive(Debug)]
#[napi(object)]
pub struct Entry {
  /// A unique identifier for this item with a feed. If not supplied it is initialised to a hash of the first link or a UUID if not available.
  /// * Atom (required): Identifies the entry using a universally unique and permanent URI.
  /// * RSS 2 (optional) "guid": A string that uniquely identifies the item.
  /// * RSS 1: does not specify a unique ID as a separate item, but does suggest the URI should be "the same as the link" so we use a hash of the link if found
  /// * JSON Feed: is unique for that item for that feed over time.
  pub id: String,
  /// Title of this item within the feed
  /// * Atom, RSS 1(required): Contains a human readable title for the entry.
  /// * RSS 2 (optional): The title of the item.
  /// * JSON Feed: The title of the item.
  pub title: Option<Text>,
  /// Time at which this item was last modified. If not provided in the source, or invalid, it is `None`.
  /// * Atom (required): Indicates the last time the entry was modified in a significant way.
  /// * RSS doesn't specify this field.
  /// * JSON Feed: the last modification date of this item
  pub updated: Option<JsDate>,

  /// Authors of this item
  /// * Atom (recommended): Collection of authors defined at the entry level.
  /// * RSS 2 (optional): Email address of the author of the item.
  /// * JSON Feed: the author of the item
  pub authors: Vec<Person>,
  /// The content of the item
  /// * Atom (recommended): Contains or links to the complete content of the entry.
  /// * RSS 2 (optional) "content:encoded": The HTML form of the content
  /// * JSON Feed: the html content of the item, or the text content if no html is specified
  pub content: Option<Content>,
  /// Links associated with this item
  /// * Atom (recommended): Identifies a related Web page.
  /// * RSS 2 (optional): The URL of the item.
  /// * RSS 1 (required): The item's URL.
  /// * JSON Feed: the url and external URL for the item is the first items, then each subsequent attachment
  pub links: Vec<Link>,
  /// A short summary of the item
  /// * Atom (recommended): Conveys a short summary, abstract, or excerpt of the entry.
  /// * RSS 1 (optional): Populated from the RSS namespace 'description' field, or if not present, the Dublin Core namespace 'description' field.
  /// * RSS 2 (optional): Populated from the RSS namespace 'description' field.
  /// * JSON Feed: the summary for the item, or the text content if no summary is provided and both text and html content are specified
  ///
  /// Warning: Some feeds (especially RSS) use significant whitespace in this field even in cases where it should be considered HTML. Consider rendering this field in a way that preserves whitespace-based formatting such as a double-newline to separate paragraphs.
  pub summary: Option<Text>,

  /// Structured classification of the item
  /// * Atom (optional): Specifies a category that the entry belongs to. A feed may have multiple category elements.
  /// * RSS 2 (optional): Includes the item in one or more categories.
  /// * JSON Feed: the supplied item tags
  pub categories: Vec<Category>,
  /// Atom (optional): Names one contributor to the entry. A feed may have multiple contributor elements.
  pub contributors: Vec<Person>,
  /// Time at which this item was first published
  /// * Atom (optional): Contains the time of the initial creation or first availability of the entry.
  /// * RSS 2 (optional) "pubDate": Indicates when the item was published.
  /// * JSON Feed: the date at which the item was published
  pub published: Option<JsDate>,
  /// Atom (optional): If an entry is copied from one feed into another feed, then this contains the source feed metadata.
  pub source: Option<String>,
  /// Atom (optional): Conveys information about rights, e.g. copyrights, held in and over the feed.
  pub rights: Option<Text>,

  /// Extension for MediaRSS - https://www.rssboard.org/media-rss
  /// A MediaObject will be created in two cases:
  /// 1) each "media:group" element encountered in the feed
  /// 2) a default for any other "media:*" elements found at the item level
  /// See the Atom tests for youtube and newscred for examples
  pub media: Vec<MediaObject>,
}

/// Represents the category of a feed or entry
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#category
/// [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html#ltcategorygtSubelementOfLtitemgt
#[derive(Debug)]
#[napi(object)]
pub struct Category {
  /// The category as a human readable string
  /// * Atom (required): Identifies the category.
  /// * RSS 2: The value of the element is a forward-slash-separated string that identifies a hierarchic location in the indicated taxonomy. Processors may establish conventions for the interpretation of categories.
  /// * JSON Feed: the value of the tag
  pub term: String,
  /// Atom (optional): Identifies the categorization scheme via a URI.
  pub scheme: Option<String>,
  /// Atom (optional): Provides a human-readable label for display.
  pub label: Option<String>,
}

/// Content, or link to the content, for a given entry.
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#contentElement
/// [RSS 2.0]: https://validator.w3.org/feed/docs/rss2.html#ltenclosuregtSubelementOfLtitemgt
#[derive(Debug)]
#[napi(object)]
pub struct Content {
  /// Atom
  /// * If the type attribute ends in +xml or /xml, then an xml document of this type is contained inline.
  /// * If the type attribute starts with text, then an escaped document of this type is contained inline.
  /// * Otherwise a base64 encoded document of the indicated media type is contained inline.
  pub body: Option<String>,
  /// Type of content
  /// * Atom: The type attribute is either text, html, xhtml, in which case the content element is defined identically to other text constructs.
  /// * RSS 2: Type says what its type is, a standard MIME type
  pub content_type: String,
  /// RSS 2.0: Length of the content in bytes
  pub length: Option<i64>,
  /// Source of the content
  /// * Atom: If the src attribute is present, it represents the URI of where the content can be found. The type attribute, if present, is the media type of the content.
  /// * RSS 2.0: where the enclosure is located
  pub src: Option<Link>,
}

/// Information on the tools used to generate the feed
///
/// Atom: Identifies the software used to generate the feed, for debugging and other purposes.
#[derive(Debug)]
#[napi(object)]
pub struct Generator {
  /// Atom: Additional data
  /// RSS 2: A string indicating the program used to generate the channel.
  pub content: String,
  /// Atom: Link to the tool
  pub uri: Option<String>,
  /// Atom: Tool version
  pub version: Option<String>,
}

/// Represents a a link to an image.
///
/// [Atom spec]:  http://www.atomenabled.org/developers/syndication/#optionalFeedElements
/// [RSS 2 spec]: https://validator.w3.org/feed/docs/rss2.html#ltimagegtSubelementOfLtchannelgt
/// [RSS 1 spec]: https://validator.w3.org/feed/docs/rss1.html#s5.4
#[derive(Debug)]
#[napi(object)]
pub struct Image {
  /// Link to the image
  /// * Atom: The URL to an image or logo
  /// * RSS 1 + 2: the URL of a GIF, JPEG or PNG image that represents the channel.
  pub uri: String,
  /// RSS 1 + 2: describes the image, it's used in the ALT attribute of the HTML <img> tag when the channel is rendered in HTML.
  pub title: Option<String>,
  /// RSS 1 + 2: the URL of the site, when the channel is rendered, the image is a link to the site.
  pub link: Option<Link>,

  /// RSS 2 (optional): width of the image
  pub width: Option<u32>,
  /// RSS 2 (optional): height of the image
  pub height: Option<u32>,
  /// RSS 2 (optional): contains text that is included in the TITLE attribute of the link formed around the image in the HTML rendering.
  pub description: Option<String>,
}

/// Represents a link to an associated resource for the feed or entry.
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#link
#[derive(Debug)]
#[napi(object)]
pub struct Link {
  /// Link to additional content
  /// * Atom: The URI of the referenced resource (typically a Web page).
  /// * RSS 2: The URL to the HTML website corresponding to the channel or item.
  /// * JSON Feed: the URI to the attachment, feed etc
  pub href: String,
  /// A single link relationship type.
  pub rel: Option<String>,
  /// Indicates the media type of the resource.
  pub media_type: Option<String>,
  /// Indicates the language of the referenced resource.
  pub href_lang: Option<String>,
  /// Human readable information about the link, typically for display purposes.
  pub title: Option<String>,
  /// The length of the resource, in bytes.
  pub length: Option<i64>,
}

/// The top-level representation of a media object
/// i.e. combines "media:*" elements from the RSS Media spec such as those under a media:group
#[derive(Debug)]
#[napi(object)]
pub struct MediaObject {
  /// Title of the object (from the media:title element)
  pub title: Option<Text>,
  /// Collection of the media content elements
  pub content: Vec<MediaContent>,
  /// Duration of the object
  pub duration: Option<i64>,
  /// Representative images for the object (from media:thumbnail elements)
  pub thumbnails: Vec<MediaThumbnail>,
  /// A text transcript, closed captioning or lyrics of the media content.
  pub texts: Vec<MediaText>,
  /// Short description of the media object (from the media:description element)
  pub description: Option<Text>,
  /// Community info (from the media:community element)
  pub community: Option<MediaCommunity>,
  /// Credits
  pub credits: Vec<MediaCredit>,
}

/// Represents a "media:community" item from the RSS Media spec
#[derive(Debug)]
#[napi(object)]
pub struct MediaCommunity {
  /// Star rating
  pub stars_avg: Option<f64>,
  pub stars_count: Option<i32>,
  pub stars_min: Option<i32>,
  pub stars_max: Option<i32>,

  /// Statistics on engagement
  pub stats_views: Option<i32>,
  pub stats_favorites: Option<i32>,
}

/// Represents a "media:content" item from the RSS Media spec
#[derive(Debug)]
#[napi(object)]
pub struct MediaContent {
  /// The direct URL
  pub url: Option<String>,
  /// Standard MIME type
  pub content_type: Option<String>,
  /// Height and width
  pub height: Option<u32>,
  pub width: Option<u32>,
  /// Duration the media plays
  pub duration: Option<i64>,
  /// Size of media in bytes
  pub size: Option<i64>,
  /// Rating
  pub rating: Option<MediaRating>,
}

/// Represents a "media:credit" item from the RSS Media spec
#[derive(Debug)]
#[napi(object)]
pub struct MediaCredit {
  /// The entity being credited
  pub entity: String,
}

/// Rating of the feed, item or media within the content
#[derive(Debug)]
#[napi(object)]
pub struct MediaRating {
  // The scheme (defaults to "simple" per the spec)
  pub urn: String,
  // The rating text
  pub value: String,
}

/// Represents a "media:text" item from the RSS Media spec
#[derive(Debug)]
#[napi(object)]
pub struct MediaText {
  /// The text
  pub text: Text,
  /// The start time offset that the text starts being relevant to the media object.
  pub start_time: Option<i64>,
  /// The end time that the text is relevant. If this attribute is not provided, and a start time is used, it is expected that the end time is either the end of the clip or the start of the next <media:text> element.
  pub end_time: Option<i64>,
}

/// Represents a "media:thumbnail" item from the RSS Media spec
#[derive(Debug)]
#[napi(object)]
pub struct MediaThumbnail {
  /// The thumbnail image
  pub image: Image,
  /// The time this thumbnail represents
  pub time: Option<i64>,
}

/// Represents an author, contributor etc.
///
/// [Atom spec]: http://www.atomenabled.org/developers/syndication/#person
#[derive(Debug)]
#[napi(object)]
pub struct Person {
  /// Atom: human-readable name for the person.
  /// JSON Feed: human-readable name for the person.
  pub name: String,
  /// Atom: home page for the person.
  /// JSON Feed: link to media (Twitter etc) for the person
  pub uri: Option<String>,
  /// Atom: An email address for the person.
  pub email: Option<String>,
}

/// Textual content, or link to the content, for a given entry.
#[derive(Debug)]
#[napi(object)]
pub struct Text {
  pub content_type: String,
  pub src: Option<String>,
  pub content: String,
}

impl Feed {
  pub fn from(env: Env, feed: model::Feed) -> Self {
    Self {
      feed_type: FeedType::from(feed.feed_type),
      id: feed.id,
      title: feed.title.map(|title| Text::from(title)),
      updated: feed.updated.map(|updated| {
        f64::from_i64(updated.timestamp_millis())
          .and_then(|timestamp| env.create_date(timestamp).ok())
          .unwrap()
      }),
      authors: feed
        .authors
        .into_iter()
        .map(|author| Person::from(author))
        .collect(),
      description: feed.description.map(|description| Text::from(description)),
      links: feed
        .links
        .into_iter()
        .map(|link| Link::from(link))
        .collect(),
      categories: feed
        .categories
        .into_iter()
        .map(|category| Category::from(category))
        .collect(),
      contributors: feed
        .contributors
        .into_iter()
        .map(|contributor| Person::from(contributor))
        .collect(),
      generator: feed.generator.map(|generator| Generator::from(generator)),
      icon: feed.icon.map(|icon| Image::from(icon)),
      language: feed.language,
      logo: feed.logo.map(|logo| Image::from(logo)),
      published: feed.published.map(|published| {
        f64::from_i64(published.timestamp_millis())
          .and_then(|timestamp| env.create_date(timestamp).ok())
          .unwrap()
      }),
      rating: feed.rating.map(|rating| MediaRating::from(rating)),
      rights: feed.rights.map(|right| Text::from(right)),
      ttl: feed.ttl,
      entries: feed
        .entries
        .into_iter()
        .map(|entry| Entry::from(env, entry))
        .collect(),
    }
  }
}

impl From<model::Generator> for Generator {
  fn from(value: model::Generator) -> Self {
    Self {
      content: value.content,
      uri: value.uri,
      version: value.version,
    }
  }
}

impl From<model::Link> for Link {
  fn from(value: model::Link) -> Self {
    Self {
      title: value.title,
      href: value.href,
      length: value.length.map(|length| i64::from_u64(length).unwrap()),
      href_lang: value.href_lang,
      media_type: value.media_type,
      rel: value.rel,
    }
  }
}

impl From<model::Category> for Category {
  fn from(value: model::Category) -> Self {
    Self {
      term: value.term,
      scheme: value.scheme,
      label: value.label,
    }
  }
}

impl From<model::Person> for Person {
  fn from(value: model::Person) -> Self {
    Self {
      name: value.name,
      uri: value.uri,
      email: value.email,
    }
  }
}

impl From<model::FeedType> for FeedType {
  fn from(value: model::FeedType) -> Self {
    match value {
      model::FeedType::Atom => FeedType::Atom,
      model::FeedType::JSON => FeedType::JSON,
      model::FeedType::RSS0 => FeedType::RSS0,
      model::FeedType::RSS1 => FeedType::RSS1,
      model::FeedType::RSS2 => FeedType::RSS2,
    }
  }
}

impl From<model::Text> for Text {
  fn from(value: model::Text) -> Self {
    Self {
      content: value.content,
      src: value.src,
      content_type: value.content_type.to_string(),
    }
  }
}

impl Entry {
  pub fn from(env: Env, value: model::Entry) -> Self {
    Self {
      id: value.id,
      title: value.title.map(|title| Text::from(title)),
      updated: value.updated.map(|updated| {
        f64::from_i64(updated.timestamp_millis())
          .and_then(|timestamp| env.create_date(timestamp).ok())
          .unwrap()
      }),
      authors: value
        .authors
        .into_iter()
        .map(|author| Person::from(author))
        .collect(),
      content: value.content.map(|content| Content::from(content)),
      links: value
        .links
        .into_iter()
        .map(|link| Link::from(link))
        .collect(),
      summary: value.summary.map(|summary| Text::from(summary)),
      categories: value
        .categories
        .into_iter()
        .map(|category| Category::from(category))
        .collect(),
      contributors: value
        .contributors
        .into_iter()
        .map(|contributor| Person::from(contributor))
        .collect(),
      published: value.published.map(|published| {
        f64::from_i64(published.timestamp_millis())
          .and_then(|timestamp| env.create_date(timestamp).ok())
          .unwrap()
      }),
      source: value.source,
      rights: value.rights.map(|right| Text::from(right)),
      media: value
        .media
        .into_iter()
        .map(|media| MediaObject::from(media))
        .collect(),
    }
  }
}

impl From<model::Image> for Image {
  fn from(value: model::Image) -> Self {
    Self {
      uri: value.uri,
      description: value.description,
      height: value.height,
      link: value.link.map(|link| Link::from(link)),
      title: value.title,
      width: value.width,
    }
  }
}

impl From<model::MediaContent> for MediaContent {
  fn from(value: model::MediaContent) -> Self {
    Self {
      width: value.width,
      height: value.height,
      rating: value.rating.map(|rating| MediaRating::from(rating)),
      content_type: value
        .content_type
        .map(|content_type| content_type.to_string()),
      duration: value
        .duration
        .map(|duration| i64::from_u64(duration.as_secs()).unwrap()),
      size: value.size.map(|size| i64::from_u64(size).unwrap()),
      url: value.url.map(|url| url.to_string()),
    }
  }
}

impl From<model::MediaRating> for MediaRating {
  fn from(value: model::MediaRating) -> Self {
    Self {
      urn: value.urn,
      value: value.value,
    }
  }
}

impl From<model::Content> for Content {
  fn from(value: model::Content) -> Self {
    Self {
      content_type: value.content_type.to_string(),
      body: value.body,
      length: value.length.map(|length| i64::from_u64(length).unwrap()),
      src: value.src.map(|src| Link::from(src)),
    }
  }
}

impl From<model::MediaObject> for MediaObject {
  fn from(value: model::MediaObject) -> Self {
    Self {
      title: value.title.map(|title| Text::from(title)),
      content: value.content.into_iter().map(MediaContent::from).collect(),
      duration: value
        .duration
        .map(|duration| i64::from_u128(duration.as_millis()).unwrap()),
      thumbnails: value
        .thumbnails
        .into_iter()
        .map(|thumbnail| MediaThumbnail::from(thumbnail))
        .collect(),
      texts: value
        .texts
        .into_iter()
        .map(|text| MediaText::from(text))
        .collect(),
      description: value.description.map(|description| Text::from(description)),
      credits: value
        .credits
        .into_iter()
        .map(|credit| MediaCredit::from(credit))
        .collect(),
      community: value
        .community
        .map(|community| MediaCommunity::from(community)),
    }
  }
}

impl From<model::MediaCommunity> for MediaCommunity {
  fn from(value: model::MediaCommunity) -> Self {
    Self {
      stars_avg: value.stars_avg,
      stars_count: value
        .stars_count
        .map(|stars_count| i32::from_u64(stars_count).unwrap()),
      stars_min: value
        .stars_min
        .map(|stars_min| i32::from_u64(stars_min).unwrap()),
      stars_max: value
        .stars_max
        .map(|stars_max| i32::from_u64(stars_max).unwrap()),
      stats_views: value
        .stats_views
        .map(|stats_views| i32::from_u64(stats_views).unwrap()),
      stats_favorites: value
        .stats_favorites
        .map(|stats_favorites| i32::from_u64(stats_favorites).unwrap()),
    }
  }
}

impl From<model::MediaText> for MediaText {
  fn from(value: model::MediaText) -> Self {
    Self {
      text: Text::from(value.text),
      end_time: value
        .end_time
        .map(|end_time| i64::from_u128(end_time.as_millis()).unwrap()),
      start_time: value
        .start_time
        .map(|start_time| i64::from_u128(start_time.as_millis()).unwrap()),
    }
  }
}

impl From<model::MediaThumbnail> for MediaThumbnail {
  fn from(value: model::MediaThumbnail) -> Self {
    Self {
      image: Image::from(value.image),
      time: value
        .time
        .map(|time| i64::from_u128(time.as_millis()).unwrap()),
    }
  }
}

impl From<model::MediaCredit> for MediaCredit {
  fn from(value: model::MediaCredit) -> Self {
    Self {
      entity: value.entity,
    }
  }
}
