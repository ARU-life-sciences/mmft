use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use regex::Regex;
use std::io;
use std::path::Path;

pub fn regex_sequences(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");
    let re_str = matches.value_of("regex").unwrap();

    let re = Regex::new(re_str);

    let re = match re {
        Ok(r) => r,
        Err(e) => {
            let err = format!(
                "{}\nError: [-]\tActual error: {}",
                error::RegexError::CouldNotCompile,
                e
            );
            bail!(err)
        }
    };

    // writer here?
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f {
                let basename = Path::new(el).file_name().unwrap().to_str().unwrap();

                let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
                for record in reader.records() {
                    let record = record.expect("[-]\tError during fasta record parsing.");
                    let id = record.id();

                    if re.is_match(id) {
                        writer
                            .write(id, Some(basename), record.seq())
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
                    if re.is_match(id) {
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
