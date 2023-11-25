use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::io;
use std::path::Path;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn filter_sequences(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let id_file = matches.get_one::<String>("file").unwrap();
    // just read file into memory for ease...
    let ids = lines_from_file(id_file);
    // writer here?
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let reader = fasta::Reader::from_file(el)?;
                for record in reader.records() {
                    let record = record?;
                    let id = record.id();

                    if ids.contains(&id.to_owned()) {
                        writer
                            .write(id, Some(&basename), record.seq())
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                    }
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let id = record.id();
                    if ids.contains(&id.to_owned()) {
                        writer
                            .write(id, None, record.seq())
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                    }
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
