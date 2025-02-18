use serde::Deserialize;
use serde::Serialize;

use crate::result::Result;

pub trait Marshall {
    fn encode<T>(value: T) -> Result<String>
    where
        T: Serialize;

    fn decode<'a, T>(value: &'a str) -> Result<T>
    where
        T: Deserialize<'a>;
}
