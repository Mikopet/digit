#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    #[test]
    fn eq() {
        assert_eq!(Digit::One, Digit::One);
        assert_ne!(Digit::Two, Digit::Three);
    }

    #[test]
    fn comp() {
        assert_eq!(Digit::One <= Digit::One, true);
        assert_eq!(Digit::Two < Digit::Three, true);
        assert_eq!(Digit::Four > Digit::Five, false);
    }

    #[test]
    fn ord_sort() {
        let mut a = [Digit::Four, Digit::Zero, Digit::Nine];
        a.sort();

        assert_eq!(a, [Digit::Zero, Digit::Four, Digit::Nine]);
    }

    #[test]
    fn ord_pq() {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        heap.push(Digit::One);
        heap.push(Digit::Eight);
        heap.push(Digit::Six);
        assert_eq!(heap.pop().unwrap(), Digit::Eight);
    }
}
