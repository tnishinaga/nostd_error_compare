use core::fmt::Display;

#[derive(Debug)]
pub struct ManualErrorSubA {
    location: &'static core::panic::Location<'static>,
}

impl ManualErrorSubA {
    #[track_caller]
    pub const fn new() -> Self {
        Self {
            location: core::panic::Location::caller(),
        }
    }
}

impl Display for ManualErrorSubA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub enum ManualError {
    A {
        source: ManualErrorSubA,
        location: &'static core::panic::Location<'static>,
    },
}

impl From<ManualErrorSubA> for ManualError {
    #[track_caller]
    fn from(value: ManualErrorSubA) -> Self {
        Self::A {
            source: value,
            location: core::panic::Location::caller(),
        }
    }
}

impl Display for ManualError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}
