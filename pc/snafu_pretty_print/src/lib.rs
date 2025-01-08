#![no_std]

use snafu::{prelude::*, ErrorCompat, GenerateImplicitData, ResultExt};

pub struct StaticLocationRef(&'static core::panic::Location<'static>);

impl core::ops::Deref for StaticLocationRef {
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

impl snafu::GenerateImplicitData for StaticLocationRef {
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

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("Failed A"), visibility(pub))]
    A {
        #[snafu(implicit)]
        location: StaticLocationRef,
        #[snafu(source)]
        source: ErrorSubA,
    },
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.iter_chain()
            .skip(1)
            .try_for_each(|e| {
                // panic!("hoge");
                writeln!(f, "{:?}", e)
            })
            .unwrap();

        match self {
            Self::A { location, source } => {
                writeln!(f, "{}, at {:?}", self, location)?;
                write!(f, "{:?}", source)
            }
        }
    }
}

pub fn err() -> Result<(), Error> {
    let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
    let _ = err.context(ASnafu)?;
    Ok(())
}
