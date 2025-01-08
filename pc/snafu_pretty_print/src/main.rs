// SPDX-FileCopyrightText: 2025 Toshifumi Nishinaga <tnishinaga.dev@gmail.com>
// SPDX-License-Identifier: MIT

use nostd_error_snafu::err;
use snafu::{prelude::*, Location, ResultExt};

fn main() {
    // let err: Result<(), Error> = err.context(ASnafu);
    println!("{:?}", err());
}
