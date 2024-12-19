// simple reverse complement the sequence

use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use clap::ArgMatches;
use noodles_fasta::{self as fasta, Record};
use std::io;

pub fn reverse(matches: &ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    let mut writer = fasta::Writer::new(io::stdout());
    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let rec = record?;
                    let mut seq = rec.sequence().as_ref().to_vec();
                    revcomp_inplace(&mut seq);

                    let out_record = Record::new(rec.definition().to_owned(), seq.into());

                    writer.write_record(&out_record)?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(rec)) = records.next() {
                    let mut seq = rec.sequence().as_ref().to_vec();
                    revcomp_inplace(&mut seq);

                    let out_record = Record::new(rec.definition().to_owned(), seq.into());

                    writer.write_record(&out_record)?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}

fn revcomp_inplace(seq: &mut [u8]) {
    // Complement bases using a precomputed static lookup table
    static COMPLEMENT: [u8; 256] = make_complement_table();

    let len = seq.len();
    for i in 0..len / 2 {
        let (left, right) = (seq[i], seq[len - i - 1]);
        seq[i] = COMPLEMENT[right as usize];
        seq[len - i - 1] = COMPLEMENT[left as usize];
    }
    if len % 2 == 1 {
        let mid = len / 2;
        seq[mid] = COMPLEMENT[seq[mid] as usize];
    }
}

// Const function to generate the complement lookup table at compile time
const fn make_complement_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    table[b'A' as usize] = b'T';
    table[b'T' as usize] = b'A';
    table[b'C' as usize] = b'G';
    table[b'G' as usize] = b'C';
    table[b'a' as usize] = b't';
    table[b't' as usize] = b'a';
    table[b'c' as usize] = b'g';
    table[b'g' as usize] = b'c';

    // Handle unknown characters by mapping them to themselves
    let mut i = 0;
    while i < 256 {
        if table[i] == 0 {
            table[i] = i as u8;
        }
        i += 1;
    }
    table
}
