use crate::types::properties::attachment::Attachment;
use crate::types::properties::attributed_to::AttributedTo;
use crate::types::properties::audience::Audience;
use crate::types::properties::bcc::Bcc;
use crate::types::properties::bto::Bto;
use crate::types::properties::cc::Cc;
use crate::types::properties::content::Content;
use crate::types::properties::context::Context;
use crate::types::properties::duration::Duration;
use crate::types::properties::end_time::EndTime;
use crate::types::properties::generator::Generator;
use crate::types::properties::icon::Icon;
use crate::types::properties::image::Image;
use crate::types::properties::in_reply_to::InReplyTo;
use crate::types::properties::location::Location;
use crate::types::properties::media_type::MediaType;
use crate::types::properties::name::Name;
use crate::types::properties::preview::Preview;
use crate::types::properties::published::Published;
use crate::types::properties::replies::Replies;
use crate::types::properties::start_time::StartTime;
use crate::types::properties::summary::Summary;
use crate::types::properties::tag::Tag;
use crate::types::properties::to::To;
use crate::types::properties::updated::Updated;
use crate::types::properties::url::Url;

/// Represents a document of any kind.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document>
pub struct Document {
    pub r#type: DocumentType,

    // Properties from Object
    pub attachment: Option<Attachment>,
    pub attributed_to: Option<AttributedTo>,
    pub audience: Option<Audience>,
    pub content: Option<Content>,
    pub context: Option<Context>,
    pub name: Option<Name>,
    pub end_time: Option<EndTime>,
    pub generator: Option<Generator>,
    pub icon: Option<Icon>,
    pub image: Option<Image>,
    pub in_reply_to: Option<InReplyTo>,
    pub location: Option<Location>,
    pub preview: Option<Preview>,
    pub published: Option<Published>,
    pub replies: Option<Replies>,
    pub start_time: Option<StartTime>,
    pub summary: Option<Summary>,
    pub tag: Option<Tag>,
    pub updated: Option<Updated>,
    pub url: Option<Url>,
    pub to: Option<To>,
    pub bto: Option<Bto>,
    pub cc: Option<Cc>,
    pub bcc: Option<Bcc>,
    pub media_type: Option<MediaType>,
    pub duration: Option<Duration>,
}

pub enum DocumentType {
    /// Represents a document of any kind.
    Document,

    /// Represents an audio document of any kind.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audio>
    Audio,

    /// An image document of any kind
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-image>
    Image,

    /// Represents a Web Page.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page>
    Page,

    /// Represents a video document of any kind.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-video>
    Video,
}
