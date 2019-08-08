use crate::error::Error;
use id3::Tag;
use log::debug;
use std::path::PathBuf;

pub mod error;

pub fn tags_to_file_system(source: PathBuf, target: PathBuf) -> Result<(), Error> {
    let tag = Tag::read_from_path(source).map_err(Error::from)?;
    debug!("Tag");

    Ok(())
}
