use crate::utils::{error, revcomp, stdin, translate};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::fmt;
use std::io;
use std::path::Path;

enum Orientation {
    Forward,
    Reverse,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Orientation::Forward => write!(f, "forward"),
            Orientation::Reverse => write!(f, "reverse"),
        }
    }
}

pub fn six_frame_translate(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");

    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f {
                let basename = Path::new(el).file_name().unwrap().to_str().unwrap();

                let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
                for record in reader.records() {
                    // now we iterate over forward/reverse
                    let r = record?.clone();
                    for strand in [Orientation::Forward, Orientation::Reverse] {
                        for skip in 0..=2 {
                            match strand {
                                Orientation::Forward => {
                                    let id = r.id();
                                    let seq = &r.seq()[skip..];

                                    writer
                                        .write(
                                            &format!("{}-strand:{}-skip:{}", id, strand, skip),
                                            Some(basename),
                                            &translate::translate(seq),
                                        )
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                                Orientation::Reverse => {
                                    let id = r.id();
                                    let seq = revcomp::reverse_complement(&r.seq()[skip..]);

                                    writer
                                        .write(
                                            &format!("{}-strand:{}-skip:{}", id, strand, skip),
                                            Some(basename),
                                            &translate::translate(&seq),
                                        )
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                        }
                    }

                    // write to stdout
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    // now we iterate over forward/reverse
                    let r = record.clone();
                    for strand in [Orientation::Forward, Orientation::Reverse] {
                        for skip in 0..=2 {
                            match strand {
                                Orientation::Forward => {
                                    let id = r.id();
                                    let seq = &r.seq()[skip..];

                                    writer
                                        .write(
                                            &format!("{}-strand:{}-skip:{}", id, strand, skip),
                                            None,
                                            &translate::translate(seq),
                                        )
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                                Orientation::Reverse => {
                                    let id = r.id();
                                    let seq = revcomp::reverse_complement(&r.seq()[skip..]);

                                    writer
                                        .write(
                                            &format!("{}-strand:{}-skip:{}", id, strand, skip),
                                            None,
                                            &translate::translate(&seq),
                                        )
                                        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                                }
                            }
                        }
                    }
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}
