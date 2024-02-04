use digit::Digit;

fn main() -> Digit {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        panic!("no argument given");
    } else {
        Digit::try_from(&args[1]).unwrap()
    }
}
