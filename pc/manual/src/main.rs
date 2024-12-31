// SPDX-FileCopyrightText: 2025 Toshifumi Nishinaga <tnishinaga.dev@gmail.com>
// SPDX-License-Identifier: MIT

use core::fmt::Display;

#[derive(Debug)]
pub struct ErrorSubA {
    _location: &'static core::panic::Location<'static>,
}

impl ErrorSubA {
    #[track_caller]
    pub const fn new() -> Self {
        Self {
            _location: core::panic::Location::caller(),
        }
    }
}

impl Display for ErrorSubA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub enum Error {
    A {
        source: ErrorSubA,
        location: &'static core::panic::Location<'static>,
    },
}

impl From<ErrorSubA> for Error {
    #[track_caller]
    fn from(value: ErrorSubA) -> Self {
        Self::A {
            source: value,
            location: core::panic::Location::caller(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

fn err() -> Result<(), Error> {
    let err = Err(ErrorSubA::new());
    let _ = err?;
    Ok(())
}

fn main() {
    println!("{:?}", err());
}
