#[allow(unused)]
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

#[allow(unused)]
fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::compute_digest;

    #[test]
    // cargo test digest -- --nocapture
    fn test_compute_digest() {
        let res = compute_digest("Rust");
        println!("{:?}", res);
        assert_eq!(res[0], 240);
    }
}
