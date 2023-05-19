use std::{fs, path::Path, str::FromStr, error::Error};

use anyhow::Result;

pub fn read_subpath<T>(path: &Path, sub: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: 'static + Error + Send + Sync,
{
    Ok(T::from_str(&fs::read_to_string(path.join(sub))?.trim())?)
}
