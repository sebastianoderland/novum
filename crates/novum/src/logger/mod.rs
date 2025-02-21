pub mod logger;
pub mod sink;
pub mod sinks;

pub(crate) mod prelude {
    pub use super::logger::*;
    pub use super::sinks::*;

    pub use log::debug;
    pub use log::error;
    pub use log::info;
    pub use log::trace;
    pub use log::warn;
}

// init_mod!{
//     files => [
//         logger,
//         sink,
//     ],
//     modules => [
//         sinks,
//     ],
// };
