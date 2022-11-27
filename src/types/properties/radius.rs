use crate::types::errors::TypeError;

pub const RADIUS_MIN_BOUND: f32 = 0.0;

/// The radius from the given latitude and longitude for a Place. The units is expressed by the units property. If units is not specified, the default is assumed to be "m" indicating "meters".
/// Specifications: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-radius
#[derive(Debug, PartialEq)]
pub struct Radius(f32);

impl Radius {
    fn new(value: f32) -> Result<Self, TypeError> {
        if value < RADIUS_MIN_BOUND {
            return Err(TypeError::OutOfBoundsMin {
                min: RADIUS_MIN_BOUND.to_string(),
                found: value.to_string(),
            });
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let radius = Radius::new(42.13);

        assert!(radius.is_ok());
        assert_eq!(radius.unwrap(), Radius(42.13));
    }

    #[test]
    fn test_new_out_of_bounds_min() {
        let radius = Radius::new(-13.9);

        assert!(radius.is_err());
        assert_eq!(
            radius.unwrap_err(),
            TypeError::OutOfBoundsMin {
                min: RADIUS_MIN_BOUND.to_string(),
                found: "-13.9".to_string()
            }
        );
    }
}
