use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::io;
use std::path::Path;

pub fn get_lengths(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f {
                let basename = Path::new(el).file_name().unwrap().to_str().unwrap();

                let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
                for record in reader.records() {
                    let record = record.expect("[-]\tError during fasta record parsing.");
                    let id = record.id();
                    let len = record.seq().len();
                    // write to stdout
                    println!("{}\t{}\t{}", basename, id, len);
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let id = record.id();
                    let len = record.seq().len();
                    // write to stdout
                    println!("{}\t{}", id, len);
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
