use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use noodles_fasta::{self as fasta, record::Definition};
use std::collections::HashSet;
use std::io;
use std::path::Path;
use std::{
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};

fn ids_from_file(filename: impl AsRef<Path>) -> Result<HashSet<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines().collect::<Result<_, _>>().map_err(Into::into)
}

pub fn filter_sequences(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let id_file = matches.get_one::<String>("file").unwrap();
    // read the id list into a set once, so each record lookup below is O(1)
    // instead of a linear scan
    let ids = ids_from_file(id_file)?;
    let mut writer = fasta::Writer::new(BufWriter::new(io::stdout()));

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;

                    let id = String::from_utf8(record.name().to_vec())?;
                    if ids.contains(&id) {
                        let description = match record.description() {
                            Some(d) => {
                                Some(format!("{}:{}", basename, std::str::from_utf8(d)?).into_bytes())
                            }
                            None => None,
                        };
                        let definition = Definition::new(id, description);

                        let record = fasta::Record::new(definition, record.sequence().to_owned());

                        writer
                            .write_record(&record)
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                    }
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::io::Reader::new(BufReader::new(io::stdin()));

                let mut records = records.records();
                while let Some(Ok(record)) = records.next() {
                    let id = String::from_utf8(record.name().to_vec())?;
                    if ids.contains(&id) {
                        writer
                            .write_record(&record)
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                    }
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    writer.get_mut().flush()?;
    Ok(())
}
