#[non_exhaustive]
#[derive(Debug)]
pub enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriminator() {
        assert_eq!(0, Digit::Zero as u8);
        assert_eq!(1, Digit::One as u16);
        assert_eq!(2, Digit::Two as u32);
        assert_eq!(3, Digit::Three as u64);
        assert_eq!(4, Digit::Four as u128);
        assert_eq!(5, Digit::Five as i8);
        assert_eq!(6, Digit::Six as i16);
        assert_eq!(7, Digit::Seven as i32);
        assert_eq!(8, Digit::Eight as i64);
        assert_eq!(9, Digit::Nine as i128);
    }
}
