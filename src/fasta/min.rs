use crate::utils::{error, lex_min::lex_min, stdin};
use anyhow::{bail, Result};
use noodles_fasta::{
    self as fasta,
    record::{Definition, Sequence},
    Record,
};
use std::io;

pub fn min(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    let minimal_rotation = lex_min(record.sequence().as_ref())?;

                    let definition =
                        Definition::new(record.name(), record.description().map(Into::into));
                    let out_record = Record::new(
                        definition,
                        Sequence::from(minimal_rotation.as_bytes().to_vec()),
                    );
                    writer
                        .write_record(&out_record)
                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    let minimal_rotation = lex_min(record.sequence().as_ref())?;
                    let definition =
                        Definition::new(record.name(), record.description().map(Into::into));
                    let out_record = Record::new(
                        definition,
                        Sequence::from(minimal_rotation.as_bytes().to_vec()),
                    );
                    writer
                        .write_record(&out_record)
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
