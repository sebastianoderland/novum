#[cfg(feature = "marshall-json")]
pub mod json;

#[cfg(feature = "marshall-yaml")]
pub mod yaml;

#[cfg(feature = "marshall-toml")]
pub mod toml;

pub mod prelude {
    #[cfg(feature = "marshall-json")]
    pub use super::json::*;

    #[cfg(feature = "marshall-yaml")]
    pub use super::yaml::*;

    #[cfg(feature = "marshall-toml")]
    pub use super::toml::*;
}
