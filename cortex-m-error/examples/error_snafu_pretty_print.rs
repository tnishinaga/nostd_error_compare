// SPDX-FileCopyrightText: 2025 Toshifumi Nishinaga <tnishinaga.dev@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]
#![no_std]

use core::ops::Deref;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use snafu::{prelude::*, ErrorCompat, GenerateImplicitData};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    hprintln!("panic!");
    hprintln!("{}", info.message());

    debug::exit(debug::EXIT_FAILURE);

    loop {}
}

pub struct StaticLocationRef(&'static core::panic::Location<'static>);

impl Deref for StaticLocationRef {
    type Target = core::panic::Location<'static>;
    fn deref(&self) -> &core::panic::Location<'static> {
        &self.0
    }
}

impl core::fmt::Debug for StaticLocationRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.file(), self.line(), self.column())
    }
}

impl GenerateImplicitData for StaticLocationRef {
    #[track_caller]
    fn generate() -> Self {
        Self(core::panic::Location::caller())
    }
}

#[derive(Snafu)]
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

impl core::fmt::Debug for ErrorSubA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\n{}, at {:?}", self, self.location)?;

        if let Some(e) = self.iter_chain().nth(1) {
            core::fmt::Debug::fmt(e, f)?;
        }

        Ok(())
    }
}

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("Failed A"), visibility(pub), context(false))]
    A {
        #[snafu(implicit)]
        location: StaticLocationRef,
        #[snafu(source)]
        source: ErrorSubA,
    },
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::A { location, source } => {
                let _ = source;
                write!(f, "\n{}, at {:?}", self, location)?;
            }
        }

        if let Some(e) = self.iter_chain().nth(1) {
            core::fmt::Debug::fmt(e, f)?;
        }

        Ok(())
    }
}

fn err() -> Result<(), Error> {
    let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
    let _ = err?;
    Ok(())
}

// fn show_error_trace(e: &(impl ErrorCompat + AsErrorSource)) {
//     for (i, e) in e.iter_chain().enumerate() {
//         hprintln!("{}: {:?}", i, e)
//     }
// }

#[entry]
fn main() -> ! {
    let err = err();
    hprintln!("sizeof(err) = {}", core::mem::size_of_val(&err));

    // if let Err(e) = &err {
    //     show_error_trace(e);
    // }

    let _e = err.unwrap();

    // hprintln!("{:?}", err);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
