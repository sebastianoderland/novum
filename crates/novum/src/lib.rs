#![allow(unused)]

#[macro_use]
extern crate novum_proc_macros;

#[macro_use]
mod macros;

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "marshall")]
pub mod marshall;

#[cfg(feature = "result")]
pub mod result;

#[cfg(feature = "time")]
pub mod time;

pub mod prelude {
    expose_mod!(logger, ["logger"]);
    expose_mod!(marshall, ["marshall"]);
    expose_mod!(result, ["result"]);
    expose_mod!(time, ["time"]);
}
