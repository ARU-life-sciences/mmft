use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::io;
use std::path::Path;

pub fn get_number_seq_bases(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f {
                let basename = Path::new(el).file_name().unwrap().to_str().unwrap();

                let mut reader = fasta::Reader::from_file(el)
                    .expect("[-]\tPath invalid.")
                    .records();
                let mut nb_reads = 0;
                let mut nb_bases = 0;

                while let Some(Ok(record)) = reader.next() {
                    nb_reads += 1;
                    nb_bases += record.seq().len();
                }
                // to stdout
                println!("{}\t{}\t{}", basename, nb_reads, nb_bases)
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                let mut nb_reads = 0;
                let mut nb_bases = 0;

                while let Some(Ok(record)) = records.next() {
                    nb_reads += 1;
                    nb_bases += record.seq().len();
                }
                // to stdout
                println!("{}\t{}", nb_reads, nb_bases)
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
