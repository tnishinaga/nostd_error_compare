// SPDX-FileCopyrightText: 2025 Toshifumi Nishinaga <tnishinaga.dev@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use snafu::{prelude::*, GenerateImplicitData, ResultExt};

#[derive(Debug)]
pub struct StaticLocationRef(&'static core::panic::Location<'static>);

impl AsRef<core::panic::Location<'static>> for StaticLocationRef {
    fn as_ref(&self) -> &'static core::panic::Location<'static> {
        &self.0
    }
}

impl GenerateImplicitData for StaticLocationRef {
    #[track_caller]
    fn generate() -> Self {
        Self(core::panic::Location::caller())
    }
}

#[derive(Debug, Snafu)]
pub struct ErrorSubA {
    #[snafu(implicit)]
    location: StaticLocationRef,
}

impl ErrorSubA {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            location: StaticLocationRef::generate(),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(""), visibility(pub))]
    A {
        #[snafu(implicit)]
        location: StaticLocationRef,
        #[snafu(source)]
        source: ErrorSubA,
    },
}

fn err() -> Result<(), Error> {
    let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
    let _ = err.context(ASnafu)?;
    Ok(())
}

#[entry]
fn main() -> ! {
    let err = err();
    hprintln!("sizeof(err) = {}", core::mem::size_of_val(&err));
    hprintln!("{:?}", err);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
