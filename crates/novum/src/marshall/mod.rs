pub mod formats;
pub mod functions;
pub mod marshall;
pub mod value;

pub(crate) mod prelude {
    pub use super::formats::prelude::*;
    pub use super::functions::*;

    pub use serde;
    pub use serde::Deserialize;
    pub use serde::Serialize;
}

// init_mod!{
//     files => [
//         functions,
//         marshall,
//         value,
//     ],
//     modules => [
//         formats,
//     ],
// };
