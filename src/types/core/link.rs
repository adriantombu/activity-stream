use crate::types::errors::TypeError;
use crate::types::properties::height::Height;
use crate::types::properties::href::Href;
use crate::types::properties::hreflang::Hreflang;
use crate::types::properties::media_type::MediaType;
use crate::types::properties::name::Name;
use crate::types::properties::preview::Preview;
use crate::types::properties::rel::Rel;
use crate::types::properties::width::Width;

/// A `Link` is an indirect, qualified reference to a resource identified by a URL. The fundamental
/// model for links is established by [RFC5988](https://www.w3.org/TR/activitystreams-vocabulary/#bib-RFC5988).
///
/// Many of the properties defined by the Activity Vocabulary allow values that are either instances
/// of [Object](crate::types::core::object::Object) or `Link`. When a `Link` is used, it establishes a qualified
/// relation connecting the subject (the containing object) to the resource identified by the [href](crate::types::properties::href::Href).
/// Properties of the `Link` are properties of the reference as opposed to properties of the resource.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-link>
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Link {
    pub r#type: LinkType,

    pub href: Option<Href>,
    pub rel: Option<Rel>,
    pub media_type: Option<MediaType>,
    pub name: Option<Name>,
    pub hreflang: Option<Hreflang>,
    pub height: Option<Height>,
    pub width: Option<Width>,
    pub preview: Option<Preview>,
}

/// A valid url string
impl TryFrom<&str> for Link {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            href: Some(Href::new(value)?),
            ..Default::default()
        })
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub enum LinkType {
    #[default]
    Link,

    /// A specialized [Link](crate::types::core::link::Link) that represents an @mention.
    ///
    /// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mention>
    Mention,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_string() {
        let link: Result<Link, TypeError> = "https://example.org/abc".try_into();

        assert!(link.is_ok());
        assert_eq!(
            link.unwrap(),
            Link {
                href: Some(Href::new("https://example.org/abc").unwrap()),
                ..Default::default()
            }
        );
    }
}
