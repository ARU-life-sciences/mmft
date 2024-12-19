use anyhow::{bail, Result};
use noodles_fasta::{io::Reader, Record};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

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

pub(crate) fn fasta_reader_file(path: PathBuf) -> Result<Reader<BufReader<File>>> {
    Ok(File::open(path).map(BufReader::new).map(Reader::new)?)
}

pub(crate) fn fasta_reader_stdin() -> Reader<BufReader<std::io::Stdin>> {
    Reader::new(BufReader::new(std::io::stdin()))
}

pub(crate) enum FID {
    Id,
    Description,
    Both(String),
}

pub(crate) fn fasta_id_description(record: &Record, fid: FID) -> Result<String> {
    match fid {
        FID::Id => Ok(String::from_utf8(record.name().to_vec())?),
        FID::Description => Ok(String::from_utf8(
            record.description().map(|e| e.to_vec()).unwrap_or_default(),
        )?),

        FID::Both(s) => Ok(format!(
            "{}{}{}",
            fasta_id_description(record, FID::Id)?,
            s,
            fasta_id_description(record, FID::Description)?
        )),
    }
}
