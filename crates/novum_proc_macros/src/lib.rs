mod expose_mod;
mod init_mod;

use proc_macro::TokenStream;

/// Expose a module and its prelude contents based on a feature.
///
/// # Parameters
///
/// - The first parameter is the name of the module to expose.
/// - The second parameter is a list of features that must be enabled for the module to be exposed, defaults to an empty list.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate novum_proc_macros;
///
/// expose_mod!(time, ["time-feature"]);
///
/// ```
///
/// This will expand to:
///
/// ```
/// #[cfg(feature = "time-feature")]
/// pub use super::time;
/// #[cfg(feature = "time-feature")]
/// pub use super::time::prelude::*;
/// ```
///
#[proc_macro]
pub fn expose_mod(item: TokenStream) -> TokenStream {
    expose_mod::run(item)
}

#[proc_macro]
pub fn init_mod(item: TokenStream) -> TokenStream {
    init_mod::run(item)
}
