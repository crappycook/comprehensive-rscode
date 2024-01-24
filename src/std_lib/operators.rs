use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

type Coordinate = (i32, i32);

impl Add for Point {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1 + p2;
        assert!(p3 == (4, 6));
        assert_eq!(p3, (4, 6));
        let p4 = &Point { x: 1, y: 2 } + &Point { x: 5, y: 6 };
        assert_eq!(p4, Point { x: 6, y: 8 });
    }
}
