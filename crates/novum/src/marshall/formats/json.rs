use crate::prelude::Result;

use super::super::marshall::Marshall;

pub struct JSON;

impl Marshall for JSON {
    fn encode<T>(value: T) -> Result<String>
    where
        T: serde::Serialize,
    {
        Ok(serde_json::to_string(&value)?)
    }

    fn decode<'a, T>(value: &'a str) -> Result<T>
    where
        T: serde::Deserialize<'a>,
    {
        Ok(serde_json::from_str(value.as_ref())?)
    }
}
