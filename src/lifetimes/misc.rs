#![allow(unused)]
#[derive(Debug)]
struct Point(i32, i32);

// Lifetimes start with ' and 'a is a typical default name.
// In this example, the the compiler does not know what lifetime to infer for p3.
// Looking inside the function body shows that it can only safely assume that p3’s lifetime is the shorter of p1 and p2
// This says, “given p1 and p2 which both outlive 'a, the return value lives for at least 'a.
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn cab_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn nearest<'a>(points: &'a [Point], query: &Point) -> Option<&'a Point> {
    let mut nearest = None;
    for p in points {
        match nearest {
            None => nearest = Some((p, cab_distance(p, query))),
            Some((_, dist)) => {
                let cd = cab_distance(p, query);
                if dist < cd {
                    nearest = Some((p, cd));
                }
            }
        }
    }
    nearest.map(|(p, _)| p)
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_most() {
        let p1 = Point(1, 2);
        let p2 = Point(3, 4);
        let result = left_most(&p1, &p2);
        assert_eq!(result, &p1);
    }

    #[test]
    fn test_nearest() {
        let n = nearest(
            &[Point(1, 0), Point(1, 0), Point(-1, 0), Point(0, -1)],
            &Point(0, 2),
        );
        assert_eq!(n, Some(&Point(1, 0)));
    }
}
