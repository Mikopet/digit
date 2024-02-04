use crate::Digit;
use std::process::ExitCode;
use std::process::Termination;

impl From<Digit> for ExitCode {
    fn from(d: Digit) -> ExitCode {
        ExitCode::from(d as u8)
    }
}

impl Termination for Digit {
    fn report(self) -> ExitCode {
        ExitCode::from(self)
    }
}
