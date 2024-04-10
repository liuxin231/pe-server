use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

#[allow(dead_code)]
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s.trim())
            .map_err(de::Error::custom)
            .map(Some),
    }
}
