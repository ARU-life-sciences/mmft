use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use noodles_fasta as fasta;
use regex::Regex;
use std::io;

pub fn regex_sequences(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let re_str = matches
        .get_one::<String>("regex")
        .expect("required by clap");
    let inverse: bool = matches.get_flag("inverse");

    let re = Regex::new(re_str)?;

    // writer here?
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    let id_desc = record.definition().to_string();

                    // if there is no match, we want to have
                    // the option to print
                    if re.is_match(&id_desc) {
                        if !inverse {
                            writer
                                .write_record(&record)
                                .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                        } else {
                            continue;
                        }
                    } else if inverse {
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
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    let id_desc = record.definition().to_string();

                    if re.is_match(&id_desc) {
                        if !inverse {
                            writer
                                .write_record(&record)
                                .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                        } else {
                            continue;
                        }
                    } else if inverse {
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
    Ok(())
}
