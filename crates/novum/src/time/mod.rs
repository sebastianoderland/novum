// pub use extern_time::*;

// pub mod datetime;
// pub mod now;

// pub(crate) mod prelude {
//     pub use super::datetime::*;
//     pub use super::now::*;
// }

init_mod! {
    files => [
        datetime,
        now,
    ],
}
