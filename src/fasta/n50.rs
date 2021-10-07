use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use io::{BufReader, Stdin};
use std::path;
use std::{fs::File, io};

pub fn get_n50(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f {
                let basename = path::Path::new(el).file_name().unwrap().to_str().unwrap();
                let reader = FFile(fasta::Reader::from_file(el).expect("[-]\tPath invalid."));
                let n50 = reader.n50();
                println!("{}\t{}", basename, n50);
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let reader = FStdin(fasta::Reader::new(io::stdin()));
                let n50 = reader.n50();
                println!("{}", n50);
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}

// based on code here:
// https://github.com/rust-bio/rust-bio-tools/blob/77012bd8af8d7a7249448f48bf7963a076b1fa71/src/sequences_stats.rs

struct FFile(fasta::Reader<BufReader<File>>);
struct FStdin(fasta::Reader<BufReader<Stdin>>);

fn inner_n50(numbers: &[usize], nb_bases_total: usize) -> usize {
    let mut acc = 0;
    for val in numbers.iter() {
        acc += *val;
        if acc > nb_bases_total / 2 {
            return *val;
        }
    }

    numbers[numbers.len() - 1]
}

impl FStdin {
    pub fn n50(self) -> usize {
        let mut lengths = Vec::new();

        let mut records = self.0.records();
        while let Some(Ok(record)) = records.next() {
            lengths.push(record.seq().len());
        }

        lengths.sort_unstable();
        let nb_bases = lengths.iter().sum::<usize>();
        inner_n50(&lengths, nb_bases)
    }
}

impl FFile {
    pub fn n50(self) -> usize {
        let mut lengths = Vec::new();

        let mut records = self.0.records();
        while let Some(Ok(record)) = records.next() {
            lengths.push(record.seq().len());
        }

        lengths.sort_unstable();
        let nb_bases = lengths.iter().sum::<usize>();
        inner_n50(&lengths, nb_bases)
    }
}
