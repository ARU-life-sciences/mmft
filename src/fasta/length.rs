use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
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

                let reader = fasta::Reader::from_file(el)?;
                for record in reader.records() {
                    let record = record?;
                    let id = record.id();
                    let desc = record.desc().unwrap_or("");
                    let len = record.seq().len();
                    // filtering
                    if let Some(l) = extract_length {
                        let length = *l;

                        match less {
                            false => {
                                // default, print greater than
                                if len > length {
                                    writer
                                        .write(
                                            id,
                                            Some(&format!("{}:length:{}", desc, len)),
                                            record.seq(),
                                        )
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                            true => {
                                // alt, print less than
                                if len < length {
                                    writer
                                        .write(
                                            id,
                                            Some(&format!("{}:length:{}", desc, len)),
                                            record.seq(),
                                        )
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
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let id = record.id();
                    let desc = record.desc().unwrap_or("");
                    let len = record.seq().len();
                    // filtering
                    if let Some(l) = extract_length {
                        let length = *l;
                        match less {
                            false => {
                                // default, print greater than
                                if len > length {
                                    writer
                                        .write(
                                            id,
                                            Some(&format!("{}:length:{}", desc, len)),
                                            record.seq(),
                                        )
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                            true => {
                                // alt, print less than
                                if len < length {
                                    writer
                                        .write(
                                            id,
                                            Some(&format!("{}:length:{}", desc, len)),
                                            record.seq(),
                                        )
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
