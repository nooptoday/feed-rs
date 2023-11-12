use feed_rs::model;
use napi_derive::napi;
use num_traits::cast::FromPrimitive;

#[derive(Debug)]
#[napi(object)]
pub struct Feed {
    pub feed_type: FeedType,
    pub id: String,
    pub updated: Option<i64>,
    // timestamp millis -> i64 -> i64
    pub authors: Vec<String>,
    pub description: Option<String>,
    // text.content
    pub links: Vec<String>,
    // links.map(|l| l.href )
    pub categories: Vec<String>,
    // category.term
    pub contributors: Vec<String>,
    // person.name
    pub generator: Option<String>,
    // generator.content
    pub icon: Option<Image>,
    pub language: Option<String>,
    pub logo: Option<Image>,
    pub published: Option<i64>,
    pub rating: Option<MediaRating>,
    pub rights: Option<String>,
    pub ttl: Option<u32>,
    pub entries: Vec<Entry>,
}

impl From<model::Feed> for Feed {
    fn from(feed: model::Feed) -> Self {
        Self {
            feed_type: FeedType::from(feed.feed_type),
            id: feed.id,
            updated: feed.updated.map(|published| { i64::from(published.timestamp_millis()) }),
            authors: feed.authors.into_iter().map(|author| { author.name }).collect(),
            description: feed.description.map(|description| { description.content }),
            links: feed.links.into_iter().map(|link| { link.href }).collect(),
            categories: feed.categories.into_iter().map(|category| { category.term })
                .collect(),
            contributors: feed.contributors.into_iter().map(|contributor| {
                contributor.name
            }).collect(),
            generator: feed.generator.map(|generator| { generator.content }),
            icon: feed.icon.map(|icon| { Image::from(icon) }),
            language: feed.language,
            logo: feed.logo.map(|logo| { Image::from(logo) }),
            published: feed.published.map(|published| {
                i64::from(published.timestamp_millis())
            }),
            rating: feed.rating.map(|rating| { MediaRating::from(rating) }),
            rights: feed.rights.map(|right| { right.content }),
            ttl: feed.ttl,
            entries: feed.entries.into_iter().map(|entry| { Entry::from(entry) }).collect(),
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
pub struct Entry {
    pub id: String,
    pub title: Option<String>,
    pub updated: Option<i64>,
    pub authors: Vec<String>,
    pub content: Option<Content>,
    pub links: Vec<String>,
    pub summary: Option<String>,
    pub categories: Vec<String>,
    pub contributors: Vec<String>,
    pub published: Option<i64>,
    pub source: Option<String>,
    pub rights: Option<String>,
    pub media: Vec<MediaObject>,
}

impl From<model::Entry> for Entry {
    fn from(value: model::Entry) -> Self {
        Self {
            id: value.id,
            title: value.title.map(|title| { title.content }),
            updated: value.updated.map(|updated| { i64::from(updated.timestamp_millis()) }),
            authors: value.authors.into_iter().map(|author| { author.name }).collect(),
            content: value.content.map(|content| { Content::from(content) }),
            links: value.links.into_iter().map(|link| { link.href }).collect(),
            summary: value.summary.map(|summary| { summary.content }),
            categories: value.categories.into_iter().map(|category| { category.term }).collect(),
            contributors: value.contributors.into_iter().map(|contributor| { contributor.name })
                .collect(),
            published: value.published.map(|published| { i64::from(published.timestamp_millis()) }),
            source: value.source,
            rights: value.rights.map(|right| { right.content }),
            media: value.media.into_iter().map(|media| { MediaObject::from(media) }).collect(),
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
            link: value.link.map(|link| { link.href }),
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
            rating: value.rating.map(|rating| { MediaRating::from(rating) }),
            content_type: value.content_type.map(|content_type| { content_type.to_string() }),
            duration: value.duration.map(|duration| {
                i64::from_u64(duration.as_secs()).unwrap()
            }),
            size: value.size.map(|size| { i64::from_u64(size).unwrap() }),
            url: value.url.map(|url| { url.to_string() }),
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
            length: value.length.map(|length| { i64::from_u64(length).unwrap() }),
            src: value.src.map(|src| { src.href }),
        }
    }
}

#[derive(Debug)]
#[napi(object)]
pub struct MediaObject {
    pub title: Option<String>,
    pub content: Vec<MediaContent>,
    pub duration: Option<i64>,
    pub thumbnails: Vec<Image>,
    pub texts: Vec<String>,
    pub description: Option<String>,
    // pub community: Option<MediaCommunity>,
    pub credits: Vec<String>,
}

impl From<model::MediaObject> for MediaObject {
    fn from(value: model::MediaObject) -> Self {
        Self {
            title: value.title.map(|title| { title.content }),
            content: value.content.into_iter().map(MediaContent::from).collect(),
            duration: value.duration.map(|duration| { i64::from_u128(duration.as_millis()).unwrap() }),
            thumbnails: value.thumbnails.into_iter().map(|thumbnail| { Image::from(thumbnail
                                                                                       .image) })
                .collect(),
            texts: value.texts.into_iter().map(|text| { text.text.content }).collect(),
            description: value.description.map(|description| { description.content }),
            credits: value.credits.into_iter().map(|credit| { credit.entity }).collect(),
        }
    }
}