use crate::{
    utils::{error, stdin},
    FID,
};
use anyhow::{bail, Result};
use noodles_fasta::{self as fasta, record::Definition, Record};
use std::io;

pub fn get_lengths(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let extract_length = matches.get_one::<usize>("extract");
    let less = matches.get_flag("less");

    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    let id = crate::fasta_id_description(&record, FID::Id)?;
                    let desc = crate::fasta_id_description(&record, FID::Description)?;
                    let len = record.sequence().len();

                    let definition = Definition::new(
                        id.clone(),
                        Some(format!("{}:length:{}", desc, len).into_bytes()),
                    );
                    // filtering
                    if let Some(l) = extract_length {
                        let length = *l;

                        match less {
                            false => {
                                // default, print greater than
                                if len > length {
                                    writer
                                        .write_record(&Record::new(
                                            definition,
                                            record.sequence().clone(),
                                        ))
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                            true => {
                                // alt, print less than
                                if len < length {
                                    writer
                                        .write_record(&Record::new(
                                            definition,
                                            record.sequence().clone(),
                                        ))
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                        }
                    } else {
                        // write to stdout
                        println!("{}\t{}\t{}", basename, id, len);
                    }
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    let id = crate::fasta_id_description(&record, FID::Id)?;
                    let desc = crate::fasta_id_description(&record, FID::Description)?;

                    let len = record.sequence().len();
                    let definition = Definition::new(
                        id.clone(),
                        Some(format!("{}:length:{}", desc, len).into_bytes()),
                    );
                    // filtering
                    if let Some(l) = extract_length {
                        let length = *l;
                        match less {
                            false => {
                                // default, print greater than
                                if len > length {
                                    writer
                                        .write_record(&Record::new(
                                            definition,
                                            record.sequence().clone(),
                                        ))
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                            true => {
                                // alt, print less than
                                if len < length {
                                    writer
                                        .write_record(&Record::new(
                                            definition,
                                            record.sequence().clone(),
                                        ))
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                        }
                    } else {
                        // write to stdout
                        println!("{}\t{}", id, len);
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
