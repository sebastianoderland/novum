use crate::result::Result;

use super::super::marshall::Marshall;

pub struct YAML;

impl Marshall for YAML {
    fn encode<T>(value: T) -> Result<String>
    where
        T: serde::Serialize,
    {
        Ok(serde_yml::to_string(&value)?)
    }

    fn decode<'a, T>(value: &'a str) -> Result<T>
    where
        T: serde::Deserialize<'a>,
    {
        Ok(serde_yml::from_str(value.as_ref())?)
    }
}
