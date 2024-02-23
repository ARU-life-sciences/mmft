use crate::utils::{error, lex_min::lex_min, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::io;

pub fn min(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let reader = fasta::Reader::from_file(el)?;
                for record in reader.records() {
                    let record = record?;
                    let id = record.id();
                    let minimal_rotation = lex_min(record.seq())?;
                    writer
                        .write(id, Some("lex_min"), minimal_rotation.as_bytes())
                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let id = record.id();
                    let minimal_rotation = lex_min(record.seq())?;
                    writer
                        .write(id, Some("lex_min"), minimal_rotation.as_bytes())
                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}
