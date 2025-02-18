#![allow(unused)]

#[macro_use]
mod macros;

if_feature!("logger", {
    pub mod logger;
});

if_feature!("marshall", {
    pub mod marshall;
});

if_feature!("result", {
    pub mod result;
});

if_feature!("result", {
    pub mod time;
});

pub mod prelude {
    if_feature!("logger", {
        pub use crate::logger::logger::*;
        pub use crate::logger::sinks::*;

        pub use log::debug;
        pub use log::error;
        pub use log::info;
        pub use log::trace;
        pub use log::warn;
    });

    if_feature!("marshall", {
        pub use crate::marshall::formats::*;
        pub use crate::marshall::functions::*;
    });

    if_feature!("result", {
        pub use crate::result::*;
    });

    if_feature!("result", {
        pub use crate::time::*;
    });
}
