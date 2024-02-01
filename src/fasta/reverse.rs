// simple reverse complement the sequence

use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::{
    alphabets::dna::revcomp,
    io::fasta::{self, Record},
};
use clap::ArgMatches;
use std::io;

pub fn reverse(matches: &ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    let mut writer = fasta::Writer::new(io::stdout());
    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let reader = fasta::Reader::from_file(el)?;
                for record in reader.records() {
                    let rec = record?;
                    let seq = rec.seq();
                    let id = rec.id();
                    let revcomp = revcomp(seq);
                    writer.write_record(&Record::with_attrs(id, Some(&basename), &revcomp))?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(rec)) = records.next() {
                    let seq = rec.seq();
                    let id = rec.id();
                    let revcomp = revcomp(seq);
                    writer.write_record(&Record::with_attrs(id, None, &revcomp))?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}
