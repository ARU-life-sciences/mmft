use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub mod fasta;
pub mod utils;

pub(crate) fn get_fasta_files(matches: &clap::ArgMatches) -> Option<Vec<PathBuf>> {
    matches.get_many("fasta").map(|e| e.cloned().collect())
}

pub(crate) fn get_basename_from_pathbuf(pb: &Path) -> Result<String> {
    match pb.file_name() {
        Some(f) => Ok(f.to_str().unwrap().to_owned()),
        None => bail!("Could not find file"),
    }
}
