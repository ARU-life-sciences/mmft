use crate::utils::{error, stdin};
use anyhow::{bail, Result};

pub fn get_n50(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                let mut lengths = Vec::new();

                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    lengths.push(record.sequence().len());
                }

                lengths.sort_unstable();
                let nb_bases = lengths.iter().sum::<usize>();
                let n50 = inner_n50(&lengths, nb_bases);
                println!("{}\t{}", basename, n50);
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut lengths = Vec::new();

                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    lengths.push(record.sequence().len());
                }

                lengths.sort_unstable();
                let nb_bases = lengths.iter().sum::<usize>();
                let n50 = inner_n50(&lengths, nb_bases);
                println!("{}", n50);
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}

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
