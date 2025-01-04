// SPDX-FileCopyrightText: 2025 Toshifumi Nishinaga <tnishinaga.dev@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use snafu::{prelude::*, Location, ResultExt};

#[derive(Debug, Snafu)]
pub struct ErrorSubA {
    #[snafu(implicit)]
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

#[entry]
fn main() -> ! {
    let err = err();
    hprintln!("sizeof(err) = {}", core::mem::size_of_val(&err));
    hprintln!("{:?}", err);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
