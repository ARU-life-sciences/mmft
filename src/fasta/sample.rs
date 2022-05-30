use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use rand::distributions::{Distribution, Uniform};
use std::io::{self, BufRead};

pub fn sample(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");
    let sample_number: i32 = matches.value_of_t("sample-number")?;

    // writer here?
    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            // total number of records
            let mut total_records = 0;
            for el in f.clone() {
                let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
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
                let bounds = Uniform::from(0..total_records);
                let mut rng = rand::thread_rng();

                let mut index = 0;
                loop {
                    let random_index: i32 = bounds.sample(&mut rng);

                    let mut inner_index = 0;
                    let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
                    let mut records = reader.records();
                    while let Some(Ok(record)) = records.next() {
                        if inner_index == random_index {
                            writer
                                .write(record.id(), Some(record.desc().unwrap_or("")), record.seq())
                                .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                            break;
                        }
                        inner_index += 1;
                    }

                    index += 1;
                    if index == sample_number {
                        break;
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

                // now second time we extract the records
                // make a random vector of usize with max upper bound
                // being the total number of records
                let bounds = Uniform::from(0..total_records);
                let mut rng = rand::thread_rng();

                let mut index = 0;
                loop {
                    let random_index: i32 = bounds.sample(&mut rng);

                    let mut inner_index = 0;

                    let mut fasta_records = fasta::Reader::new(stdin_fasta.as_bytes()).records();
                    while let Some(Ok(record)) = fasta_records.next() {
                        if inner_index == random_index {
                            writer
                                .write(record.id(), Some(record.desc().unwrap_or("")), record.seq())
                                .map_err(|_| error::FastaWriteError::CouldNotWrite)?;
                            break;
                        }
                        inner_index += 1;
                    }

                    index += 1;
                    if index == sample_number {
                        break;
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
