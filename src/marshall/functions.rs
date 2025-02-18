use crate::result::Result;

use super::marshall::Marshall;

pub fn encode<F>(value: impl serde::Serialize) -> Result<String>
where
    F: Marshall,
{
    F::encode(value)
}

pub fn decode<'a, F, T>(value: &'a str) -> Result<T>
where
    F: Marshall,
    T: serde::Deserialize<'a>,
{
    F::decode(value)
}
