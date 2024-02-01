use crate::Digit;

impl TryFrom<&str> for Digit {
    type Error = ();

    fn try_from(s: &str) -> Result<Digit, Self::Error> {
        match s {
            "0" | "zero" => Ok(Digit::Zero),
            "1" | "one" => Ok(Digit::One),
            "2" | "two" => Ok(Digit::Two),
            "3" | "three" => Ok(Digit::Three),
            "4" | "four" => Ok(Digit::Four),
            "5" | "five" => Ok(Digit::Five),
            "6" | "six" => Ok(Digit::Six),
            "7" | "seven" => Ok(Digit::Seven),
            "8" | "eight" => Ok(Digit::Eight),
            "9" | "nine" => Ok(Digit::Nine),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for Digit {
    type Error = ();

    fn try_from(s: String) -> Result<Digit, Self::Error> {
        Digit::try_from(s.as_str())
    }
}

impl TryFrom<&String> for Digit {
    type Error = ();

    fn try_from(s: &String) -> Result<Digit, Self::Error> {
        Digit::try_from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::Digit;

    #[test]
    fn from_str() {
        assert_eq!(Ok(Digit::Zero), Digit::try_from("0"));
        assert_eq!(Ok(Digit::Zero), Digit::try_from("zero"));

        assert_eq!(Ok(Digit::One), Digit::try_from("1"));
        assert_eq!(Ok(Digit::One), Digit::try_from("one"));

        assert_eq!(Ok(Digit::Two), Digit::try_from("2"));
        assert_eq!(Ok(Digit::Two), Digit::try_from("two"));

        assert_eq!(Ok(Digit::Three), Digit::try_from("3"));
        assert_eq!(Ok(Digit::Three), Digit::try_from("three"));

        assert_eq!(Ok(Digit::Four), Digit::try_from("4"));
        assert_eq!(Ok(Digit::Four), Digit::try_from("four"));

        assert_eq!(Ok(Digit::Five), Digit::try_from("5"));
        assert_eq!(Ok(Digit::Five), Digit::try_from("five"));

        assert_eq!(Ok(Digit::Six), Digit::try_from("6"));
        assert_eq!(Ok(Digit::Six), Digit::try_from("six"));

        assert_eq!(Ok(Digit::Seven), Digit::try_from("7"));
        assert_eq!(Ok(Digit::Seven), Digit::try_from("seven"));

        assert_eq!(Ok(Digit::Eight), Digit::try_from("8"));
        assert_eq!(Ok(Digit::Eight), Digit::try_from("eight"));

        assert_eq!(Ok(Digit::Nine), Digit::try_from("9"));
        assert_eq!(Ok(Digit::Nine), Digit::try_from("nine"));

        assert!(Digit::try_from("").is_err());
    }

    #[test]
    fn from_string() {
        let s = String::from("zero");
        assert_eq!(Digit::Zero, Digit::try_from(&s).unwrap());
        assert_eq!(Digit::Zero, Digit::try_from(s).unwrap());

        let s = String::from("");
        assert!(Digit::try_from(&s).is_err());
        assert!(Digit::try_from(s).is_err());
    }
}
