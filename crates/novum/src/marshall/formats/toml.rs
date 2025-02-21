use crate::prelude::Result;

use super::super::marshall::Marshall;

pub struct TOML;

impl Marshall for TOML {
    fn encode<T>(value: T) -> Result<String>
    where
        T: serde::Serialize,
    {
        Ok(toml::to_string(&value)?)
    }

    fn decode<'a, T>(value: &'a str) -> Result<T>
    where
        T: serde::Deserialize<'a>,
    {
        Ok(T::deserialize(toml::de::Deserializer::new(value.as_ref()))?)
    }
}
