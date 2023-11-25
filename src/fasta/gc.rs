use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use bio::seq_analysis::gc::gc_content;
use std::io;

pub fn get_gc(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let reader = fasta::Reader::from_file(el)?;
                for record in reader.records() {
                    let record = record?;
                    let id = record.id();
                    let gc = gc_content(record.seq());
                    // write to stdout
                    println!("{}\t{}\t{}", basename, id, gc);
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let id = record.id();
                    let gc = gc_content(record.seq());
                    // write to stdout
                    println!("{}\t{}", id, gc);
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}
