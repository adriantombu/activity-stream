use crate::types::errors::TypeError;
use url::Url;

/// Provides the globally unique identifier for an [Object](crate::types::core::object::Object) or [Link](crate::types::core::link::Link).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-id>
#[derive(Debug, PartialEq, Eq)]
pub struct Id(Url);

impl Id {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = Id::new("https://example.org/foo");

        assert!(id.is_ok());
        assert_eq!(
            id.unwrap(),
            Id(Url::parse("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let id = Id::new("example/foo");

        assert!(id.is_err());
    }
}
