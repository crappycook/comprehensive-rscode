#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    #[test]
    fn test_from_into() {
        let s = String::from("hello");
        // That’s why it is common to only implement From, as your type will get Into implementation too.
        let s_into: String = "hello".into();
        assert_eq!(s, s_into);

        let ip = Ipv4Addr::from([127, 0, 0, 1]);
        assert_eq!(ip.to_string(), "127.0.0.1");

        let num = i32::from(12_i16);
        assert!(num == 12);

        // For infallible casts (e.g. u32 to u64), prefer using From or Into over as to confirm that the cast is in fact infallible.
        // For fallible casts, TryFrom and TryInto are available when you want to handle casts that fit differently from those that don’t.
        let value = 1000;
        let n = u8::try_from(value).unwrap_or(0);
        assert_eq!(n, 0);
    }
}
