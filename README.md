digit
=====

`digit` is a very simple crate for convert one-digit decimal numbers from various types to various types.
It is created for educational purposes, but feel free to use your real-world use cases.

The project is following semver, and trying to be as compatible with itself as it is possible.
Also the crate is not using external dependencies.

## Installation

For adding to your project just simply run:
```bash
cargo add digit
```

## Features

The crate is not doing anything else, than add a `Digit` enum with the values from `Zero..Nine`
and implement a few standard traits on it.

Such as:

### TryFrom (string types)

The crate is able to understand `"0"` or `"zero"` from `&str`, `String` or `&String` and will return
the related enum variant:

```rust
assert_eq!(Ok(Digit::Zero), Digit::try_from("0"));
assert_eq!(Ok(Digit::One), Digit::try_from("one"));
```

### TryFrom (character types)

under development

### Emojis

under development

### TryFrom (number types)

under development

### Termination

The crate implements termination too, so you could return with a `Digit` in your main function:

```rust
use digit::Digit;

fn main() -> Digit {
    println!("Succesful run!");
    Digit::Zero
}
```

### Debug, PartialEq, Eq, PartialOrd, Ord

The crate implements these basic features, so you could use `Digit` in various situations:

```rust
assert_eq!(Digit::One, Digit::One);
assert_ne!(Digit::Two, Digit::Three);

assert_eq!(Digit::One <= Digit::One, true);
assert_eq!(Digit::Two < Digit::Three, true);
assert_eq!(Digit::Four > Digit::Five, false);

let mut a = [Digit::Four, Digit::Zero, Digit::Nine];
a.sort();
assert_eq!(a, [Digit::Zero, Digit::Four, Digit::Nine]);

println!("{:?}", Digit::Seven);
```

### Display

under development

## Contributing

I don't see too much space to improve this project, however, if you see something to add...
... feel free to open an issue or a PR.

