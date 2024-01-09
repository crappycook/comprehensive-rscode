// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
#[allow(unused)]
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut res = 0.0;
    for i in vector {
        res += i.powi(2);
    }
    res.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
#[allow(unused)]
fn normalize(vector: &mut [f64; 3]) {
    let magnitude = magnitude(vector);
    for i in 0..vector.len() {
        vector[i] /= magnitude;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_magnitude() {
        let v = [3.0, 4.0, 0.0];
        assert_eq!(5.0, super::magnitude(&v));

        let mut v2 = [3.0, 4.0, 0.0];
        super::normalize(&mut v2);
        assert_eq!(v2, [0.6, 0.8, 0.0]);
    }
}
