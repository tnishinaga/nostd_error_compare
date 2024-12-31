use ::snafu::{location, ResultExt};
use nostd_error::{
    manual_error,
    snafu::{self, *},
    thiserror::{self},
};

fn thiserror_test() {
    let err: thiserror::ThisError = thiserror::ThisErrorSubA::new().into();

    println!("{:?}", err);
}

fn manual_error() {
    let err: manual_error::ManualError = manual_error::ManualErrorSubA::new().into();

    println!("manual error {:?}", err);
}

fn snafu_error() {
    let err: Result<(), snafu::SnafuErrorSubA> = Err(snafu::SnafuErrorSubA::new());
    let err: Result<(), snafu::SnafuError> = err.context(FromErrorSubASnafu);

    println!("snafu error {:?}", err);
}

fn main() {
    thiserror_test();
    manual_error();
    snafu_error();

    println!("Hello, world!");
}
