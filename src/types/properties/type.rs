use crate::types::errors::TypeError;
use url::Url;

/// Identifies the [Object](crate::types::core::object::Object) or [Link](crate::types::core::link::Link) type.
/// Multiple values may be specified.
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-type>
#[derive(Debug, PartialEq, Eq)]
pub struct Type(Url);

impl Type {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Url::parse(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Type::new("https://example.org/foo");

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Type(Url::parse("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let t = Type::new("example/foo");

        assert!(t.is_err());
    }
}
