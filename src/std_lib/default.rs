#![allow(dead_code)]
#[derive(Debug, Default)]
struct Position {
    location: Location,
    name: String,
    intro: String,
}

#[derive(Debug)]
struct Location(f64, f64);

impl Default for Location {
    fn default() -> Self {
        Self(12.0, 90.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_position() {
        let default_position = Position::default();
        assert_eq!(default_position.location.0, 12.0);

        let custom_position = Position {
            name: String::from("Shanghai"),
            ..Position::default()
        };
        assert_eq!(custom_position.location.1, 90.0);
        assert_eq!(custom_position.intro, "");
    }
}
