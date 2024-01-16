// Tuple Structs
struct Point(f64, f64);

#[allow(unused)]
fn distance_between(p1: Point, p2: Point) -> f64 {
    let sum = (p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2);
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance_between() {
        let p1 = super::Point(0.0, 0.0);
        let p2 = super::Point(3.0, 4.0);
        assert_eq!(super::distance_between(p1, p2), 5.0);
    }
}
