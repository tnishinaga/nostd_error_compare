use core::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct ThisErrorSubA {
    location: &'static core::panic::Location<'static>,
}

impl ThisErrorSubA {
    #[track_caller]
    pub const fn new() -> Self {
        Self {
            location: core::panic::Location::caller(),
        }
    }
}

impl Display for ThisErrorSubA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

#[derive(Error, Debug)]
pub enum ThisError {
    #[error("")]
    A {
        #[from]
        source: ThisErrorSubA,
        // location: &'static core::panic::Location<'static>,
    },
}
