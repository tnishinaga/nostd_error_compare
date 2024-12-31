// SPDX-FileCopyrightText: 2025 Toshifumi Nishinaga <tnishinaga.dev@gmail.com>
// SPDX-License-Identifier: MIT

use snafu::{prelude::*, Location, ResultExt};

#[derive(Debug, Snafu)]
pub struct ErrorSubA {
    pub location: Location,
}

impl ErrorSubA {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            location: Location::default(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(""), visibility(pub))]
    A {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        source: ErrorSubA,
    },
}

fn err() -> Result<(), Error> {
    let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
    let _ = err.context(ASnafu)?;
    Ok(())
}

fn main() {
    // let err: Result<(), Error> = err.context(ASnafu);
    println!("{:?}", err());
}
