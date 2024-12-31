use core::fmt::Error;

use snafu::{location, prelude::*, Location, ResultExt};

#[derive(Debug, Snafu)]
pub struct SnafuErrorSubA {
    pub location: Location,
}

impl SnafuErrorSubA {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            location: Location::default(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum SnafuError {
    #[snafu(display(""), visibility(pub))]
    FromErrorSubA {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        source: SnafuErrorSubA,
    },
}
