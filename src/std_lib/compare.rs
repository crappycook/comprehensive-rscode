#![allow(dead_code)]
struct Player {
    name: String,
    position: String,
    age: u8,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.position == other.position
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.age.partial_cmp(&other.age) {
            Some(std::cmp::Ordering::Equal) => self.name.partial_cmp(&other.name),
            result => result, // it returns the result of the age comparison.
        }
    }
}

struct Key {
    id: u32,
    metadata: Option<String>,
}

// PartialEq can be implemented between different types, but Eq cannot, because it is reflexive:
impl PartialEq<u32> for Key {
    fn eq(&self, other: &u32) -> bool {
        self.id == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let p1 = Player {
            name: String::from("John"),
            position: String::from("Goalkeeper"),
            age: 30,
        };
        let p2 = Player {
            name: String::from("John"),
            position: String::from("Goalkeeper"),
            age: 29,
        };
        let p3 = Player {
            name: String::from("Mike"),
            position: String::from("Goalkeeper"),
            age: 30,
        };
        assert!(p1 == p2); // This should pass
        assert!(p1 < p3);
        assert!(p2 < p3);

        let key1 = Key {
            id: 0,
            metadata: Some(String::from("metadata1")),
        };
        let key2 = 0 as u32;
        assert!(key1 == key2);
    }
}
