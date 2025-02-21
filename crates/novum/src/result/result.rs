pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
