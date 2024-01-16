#[derive(Debug)]
#[allow(unused)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
#[allow(unused)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

#[repr(u32)]
#[allow(unused)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_enums() {
        let pm = super::PlayerMove::Run(super::Direction::Left);
        println!("{:?}", pm);

        assert_eq!(super::Bar::A as i32, 0);
        assert_eq!(super::Bar::C as i32, 10001);
    }
}
