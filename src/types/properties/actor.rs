//! {
//! "@context": "https://www.w3.org/ns/activitystreams",
//! "summary": "Sally offered the Foo object",
//! "type": "Offer",
//! "actor": "http://sally.example.org",
//! "object": "http://example.org/foo"
//! }
//!
//! {
//! "@context": "https://www.w3.org/ns/activitystreams",
//! "summary": "Sally offered the Foo object",
//! "type": "Offer",
//! "actor": {
//!     "type": "Person",
//!     "id": "http://sally.example.org",
//!     "summary": "Sally"
//!   },
//!   "object": "http://example.org/foo"
//! }
//!
//! {
//!   "@context": "https://www.w3.org/ns/activitystreams",
//!   "summary": "Sally and Joe offered the Foo object",
//!   "type": "Offer",
//!   "actor": [
//!     "http://joe.example.org",
//!     {
//!       "type": "Person",
//!       "id": "http://sally.example.org",
//!       "name": "Sally"
//!     }
//!   ],
//!   "object": "http://example.org/foo"
//! }
//!

use crate::types::core::link::Link;
use crate::types::errors::TypeError;

// TODO: handle object & lists of links/object
/// Describes one or more entities that either performed or are expected to perform the activity.
/// Any single activity can have multiple `actor`s. The `actor` MAY be specified using an indirect [Link](crate::types::core::link::Link).
///
/// Specifications: <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-actor>
#[derive(Debug, PartialEq, Eq)]
pub struct Actor(Link);

impl Actor {
    pub fn new(value: &str) -> Result<Self, TypeError> {
        Ok(Self(Link::try_from(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Actor::new("https://example.org/foo");

        assert!(t.is_ok());
        assert_eq!(
            t.unwrap(),
            Actor(Link::try_from("https://example.org/foo").unwrap())
        );
    }

    #[test]
    fn test_new_error() {
        let t = Actor::new("example/foo");

        assert!(t.is_err());
    }
}
