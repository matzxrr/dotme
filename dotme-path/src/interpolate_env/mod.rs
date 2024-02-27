mod error;

#[cfg(test)]
mod tests;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use self::error::Error;

/// Replaces ENV variables in a path with their value
///
/// yea...
pub fn interpoate_env(path: &Path) -> Result<PathBuf, Error> {
    let mut output = PathBuf::new();
    for part in path.iter() {
        let bytes = interpolate_part(part)?;
        // SAFETY: IDK Yet
        unsafe {
            output.push(OsStr::from_encoded_bytes_unchecked(bytes));
        }
    }
    Ok(output)
}

fn interpolate_part(part: &OsStr) -> Result<&[u8], Error> {
    let bytes = part.as_encoded_bytes();
    Ok(bytes)
}
