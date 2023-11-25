use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use rand::prelude::SliceRandom;
use std::io::{self, BufRead};

pub fn sample(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let sample_number = *matches
        .get_one::<i32>("sample-number")
        .expect("required by clap");

    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            // total number of records
            let mut total_records = 0;
            for el in f.clone() {
                let reader = fasta::Reader::from_file(el)?;
                // we need to iterate over the records once to get the total number of records
                for _ in reader.records() {
                    total_records += 1;
                }
            }

            if sample_number > total_records {
                bail!(
                    "[-]\tSample number ({}) cannot be greater than total number of records ({}).",
                    sample_number,
                    total_records
                );
            }

            // now iterate for real
            for el in f {
                let reader = fasta::Reader::from_file(el)?;

                let records = reader.records();

                let mut numbers: Vec<usize> = (0..total_records as usize).collect();
                numbers.shuffle(&mut rand::thread_rng());
                let mut usable_numbers = numbers[0..(sample_number as usize)].to_vec();
                usable_numbers.sort();

                for (inner_index, record) in records.enumerate() {
                    let inner_record = record?;
                    let first_random_index = match usable_numbers.first() {
                        Some(i) => i,
                        None => continue,
                    };

                    if inner_index == *first_random_index {
                        writer
                            .write(
                                inner_record.id(),
                                Some(inner_record.desc().unwrap_or("")),
                                inner_record.seq(),
                            )
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;

                        usable_numbers.remove(0);
                    }
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                // total number of records
                let mut total_records = 0;
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                // save each line of stdin to a vector
                let mut stdin_fasta = String::new();
                let mut eof = false;
                // read everything from stdin to the buffer
                while !eof {
                    match handle.read_line(&mut stdin_fasta) {
                        Ok(0) => {
                            eof = true;
                        }
                        Ok(_) => {}
                        Err(_) => {
                            bail!("[-]\tError reading from stdin.");
                        }
                    }
                }

                let fasta_reader = fasta::Reader::new(stdin_fasta.as_bytes());

                for _ in fasta_reader.records() {
                    total_records += 1;
                }

                if sample_number > total_records {
                    bail!(
                    "[-]\tSample number ({}) cannot be greater than total number of records ({}).",
                    sample_number,
                    total_records
                );
                }
                // now sample
                let reader = fasta::Reader::new(stdin_fasta.as_bytes());

                let records = reader.records();

                let mut numbers: Vec<usize> = (0..total_records as usize).collect();
                numbers.shuffle(&mut rand::thread_rng());
                let mut usable_numbers = numbers[0..(sample_number as usize)].to_vec();
                usable_numbers.sort();

                for (inner_index, record) in records.enumerate() {
                    let inner_record = record?;
                    let first_random_index = match usable_numbers.first() {
                        Some(i) => i,
                        None => continue,
                    };

                    if inner_index == *first_random_index {
                        writer
                            .write(
                                inner_record.id(),
                                Some(inner_record.desc().unwrap_or("")),
                                inner_record.seq(),
                            )
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;

                        usable_numbers.remove(0);
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
