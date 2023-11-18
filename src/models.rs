use feed_rs::model;
use napi_derive::napi;
use num_traits::cast::FromPrimitive;

#[derive(Debug)]
#[napi(object)]
pub struct Feed {
  pub feed_type: FeedType,
  pub id: String,
  pub title: Option<Text>,
  pub updated: Option<i64>, // RFC3339, ISO8601 compatible
  pub authors: Vec<Person>,
  pub description: Option<Text>,
  pub links: Vec<Link>,
  pub categories: Vec<Category>,
  pub contributors: Vec<Person>,
  pub generator: Option<Generator>,
  pub icon: Option<Image>,
  pub language: Option<String>,
  pub logo: Option<Image>,
  pub published: Option<i64>,
  pub rating: Option<MediaRating>,
  pub rights: Option<Text>,
  pub ttl: Option<u32>,
  pub entries: Vec<Entry>,
}

impl From<model::Feed> for Feed {
  fn from(feed: model::Feed) -> Self {
    Self {
      feed_type: FeedType::from(feed.feed_type),
      id: feed.id,
      title: feed.title.map(|title| Text::from(title)),
      updated: feed.updated.map(|updated| updated.timestamp_millis()),
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
      published: feed.published.map(|published| published.timestamp_millis()),
      rating: feed.rating.map(|rating| MediaRating::from(rating)),
      rights: feed.rights.map(|right| Text::from(right)),
      ttl: feed.ttl,
      entries: feed
        .entries
        .into_iter()
        .map(|entry| Entry::from(entry))
        .collect(),
    }
  }
}

#[derive(Debug)]
#[napi(object)]
pub struct Generator {
  pub content: String,
  pub uri: Option<String>,
  pub version: Option<String>,
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

#[derive(Debug)]
#[napi(object)]
pub struct Link {
  pub href: String,
  pub rel: Option<String>,
  pub media_type: Option<String>,
  pub href_lang: Option<String>,
  pub title: Option<String>,
  pub length: Option<i64>, // Originally u64, might need explicit conversion to prevent errors.
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

#[derive(Debug)]
#[napi(object)]
pub struct Category {
  pub term: String,
  pub scheme: Option<String>,
  pub label: Option<String>,
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

#[derive(Debug)]
#[napi(object)]
pub struct Person {
  pub name: String,
  pub uri: Option<String>,
  pub email: Option<String>,
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

#[derive(Debug)]
#[napi(string_enum)]
pub enum FeedType {
  Atom,
  JSON,
  RSS0,
  RSS1,
  RSS2,
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

#[derive(Debug)]
#[napi(object)]
pub struct Text {
  pub content_type: String,
  // Mime
  pub src: Option<String>,
  pub content: String,
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

#[derive(Debug)]
#[napi(object)]
pub struct Entry {
  pub id: String,
  pub title: Option<Text>,
  pub updated: Option<i64>,
  pub authors: Vec<Person>,
  pub content: Option<Content>,
  pub links: Vec<Link>,
  pub summary: Option<Text>,
  pub categories: Vec<Category>,
  pub contributors: Vec<Person>,
  pub published: Option<i64>,
  pub source: Option<String>,
  pub rights: Option<Text>,
  pub media: Vec<MediaObject>,
}

impl From<model::Entry> for Entry {
  fn from(value: model::Entry) -> Self {
    Self {
      id: value.id,
      title: value.title.map(|title| Text::from(title)),
      updated: value.updated.map(|updated| updated.timestamp_millis()),
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
      published: value
        .published
        .map(|published| published.timestamp_millis()),
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

#[derive(Debug)]
#[napi(object)]
pub struct Image {
  pub uri: String,
  pub title: Option<String>,
  pub link: Option<String>,
  pub width: Option<u32>,
  pub height: Option<u32>,
  pub description: Option<String>,
}

impl From<model::Image> for Image {
  fn from(value: model::Image) -> Self {
    Self {
      uri: value.uri,
      description: value.description,
      height: value.height,
      link: value.link.map(|link| link.href),
      title: value.title,
      width: value.width,
    }
  }
}

#[derive(Debug)]
#[napi(object)]
pub struct MediaContent {
  pub url: Option<String>,
  pub content_type: Option<String>,
  pub height: Option<u32>,
  pub width: Option<u32>,
  pub duration: Option<i64>,
  pub size: Option<i64>,
  pub rating: Option<MediaRating>,
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

#[derive(Debug)]
#[napi(object)]
pub struct MediaRating {
  pub urn: String,
  pub value: String,
}

impl From<model::MediaRating> for MediaRating {
  fn from(value: model::MediaRating) -> Self {
    Self {
      urn: value.urn,
      value: value.value,
    }
  }
}

#[derive(Debug)]
#[napi(object)]
pub struct Content {
  pub body: Option<String>,
  pub content_type: String,
  pub length: Option<i64>,
  pub src: Option<String>,
}

impl From<model::Content> for Content {
  fn from(value: model::Content) -> Self {
    Self {
      content_type: value.content_type.to_string(),
      body: value.body,
      length: value.length.map(|length| i64::from_u64(length).unwrap()),
      src: value.src.map(|src| src.href),
    }
  }
}

#[derive(Debug)]
#[napi(object)]
pub struct MediaObject {
  pub title: Option<Text>,
  pub content: Vec<MediaContent>,
  pub duration: Option<i64>,
  pub thumbnails: Vec<MediaThumbnail>,
  pub texts: Vec<MediaText>,
  pub description: Option<Text>,
  pub community: Option<MediaCommunity>,
  pub credits: Vec<MediaCredit>,
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

#[derive(Debug)]
#[napi(object)]
pub struct MediaCommunity {
  pub stars_avg: Option<f64>,
  pub stars_count: Option<i32>,
  pub stars_min: Option<i32>,
  pub stars_max: Option<i32>,
  pub stats_views: Option<i32>,
  pub stats_favorites: Option<i32>,
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

#[derive(Debug)]
#[napi(object)]
pub struct MediaText {
  pub text: Text,
  pub start_time: Option<i64>,
  pub end_time: Option<i64>,
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

#[derive(Debug)]
#[napi(object)]
pub struct MediaThumbnail {
  pub image: Image,
  pub time: Option<i64>,
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

#[derive(Debug)]
#[napi(object)]
pub struct MediaCredit {
  pub entity: String,
}

impl From<model::MediaCredit> for MediaCredit {
  fn from(value: model::MediaCredit) -> Self {
    Self {
      entity: value.entity,
    }
  }
}
