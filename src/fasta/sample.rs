use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use rand::prelude::SliceRandom;
use std::io::{self, BufRead};

pub fn sample(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let sample_number = matches.get_one::<i32>("sample-number").cloned();
    let sample_size = matches
        .get_one::<String>("sample-size")
        .map(|s| parse_size(s))
        .transpose()?; // Convert to an optional Result

    let mut writer = fasta::Writer::new(io::stdout());
    let mut total_sampled_size = 0;

    match input_file {
        Some(f) => {
            let mut total_records = 0;
            for el in f.clone() {
                let reader = fasta::Reader::from_file(el)?;
                for _ in reader.records() {
                    total_records += 1;
                }
            }

            if let Some(num) = sample_number {
                if num > total_records {
                    bail!(
                        "[-]\tSample number ({}) cannot be greater than total number of records ({}).",
                        num,
                        total_records
                    );
                }
            }

            for el in f {
                let reader = fasta::Reader::from_file(el)?;
                let records = reader.records();

                let mut numbers: Vec<usize> = (0..total_records as usize).collect();
                numbers.shuffle(&mut rand::thread_rng());
                let mut usable_numbers = match sample_number {
                    Some(num) => numbers[0..(num as usize)].to_vec(),
                    None => numbers, // Sample all if no limit
                };
                usable_numbers.sort();

                for (inner_index, record) in records.enumerate() {
                    if total_sampled_size >= sample_size.unwrap_or(usize::MAX) {
                        break;
                    }

                    let inner_record = record?;
                    let first_random_index = match usable_numbers.first() {
                        Some(i) => i,
                        None => continue,
                    };

                    if inner_index == *first_random_index {
                        let seq_size = inner_record.seq().len() + inner_record.id().len();
                        if total_sampled_size + seq_size > sample_size.unwrap_or(usize::MAX) {
                            break;
                        }

                        writer
                            .write(
                                inner_record.id(),
                                Some(inner_record.desc().unwrap_or("")),
                                inner_record.seq(),
                            )
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;

                        total_sampled_size += seq_size;
                        usable_numbers.remove(0);
                    }
                }
            }
        }
        None => match stdin::is_stdin() {
            true => {
                let mut total_records = 0;
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                let mut stdin_fasta = String::new();
                let mut eof = false;

                while !eof {
                    match handle.read_line(&mut stdin_fasta) {
                        Ok(0) => eof = true,
                        Ok(_) => {}
                        Err(_) => bail!("[-]\tError reading from stdin."),
                    }
                }

                let fasta_reader = fasta::Reader::new(stdin_fasta.as_bytes());

                for _ in fasta_reader.records() {
                    total_records += 1;
                }

                if let Some(num) = sample_number {
                    if num > total_records {
                        bail!(
                            "[-]\tSample number ({}) cannot be greater than total number of records ({}).",
                            num,
                            total_records
                        );
                    }
                }

                let reader = fasta::Reader::new(stdin_fasta.as_bytes());
                let records = reader.records();

                let mut numbers: Vec<usize> = (0..total_records as usize).collect();
                numbers.shuffle(&mut rand::thread_rng());
                let mut usable_numbers = match sample_number {
                    Some(num) => numbers[0..(num as usize)].to_vec(),
                    None => numbers,
                };
                usable_numbers.sort();

                for (inner_index, record) in records.enumerate() {
                    if total_sampled_size >= sample_size.unwrap_or(usize::MAX) {
                        break;
                    }

                    let inner_record = record?;
                    let first_random_index = match usable_numbers.first() {
                        Some(i) => i,
                        None => continue,
                    };

                    if inner_index == *first_random_index {
                        let seq_size = inner_record.seq().len() + inner_record.id().len();
                        if total_sampled_size + seq_size > sample_size.unwrap_or(usize::MAX) {
                            break;
                        }

                        writer
                            .write(
                                inner_record.id(),
                                Some(inner_record.desc().unwrap_or("")),
                                inner_record.seq(),
                            )
                            .map_err(|_| error::FastaWriteError::CouldNotWrite)?;

                        total_sampled_size += seq_size;
                        usable_numbers.remove(0);
                    }
                }
            }
            false => bail!(error::StdinError::NoSequence),
        },
    }
    Ok(())
}

fn parse_size(size: &str) -> Result<usize> {
    let size = size.trim();

    let res = if let Some(num) = size.strip_suffix("gb").or_else(|| size.strip_suffix("Gb")) {
        num.parse::<f64>()
            .map(|n| n * 1_000_000_000f64)
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    } else if let Some(num) = size.strip_suffix("mb").or_else(|| size.strip_suffix("Mb")) {
        num.parse::<f64>()
            .map(|n| n * 1_000_000f64)
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    } else if let Some(num) = size.strip_suffix("kb").or_else(|| size.strip_suffix("Kb")) {
        num.parse::<f64>()
            .map(|n| n * 1_000f64)
            .map_err(|_| anyhow::anyhow!("Invalid size fohmat"))
    } else if let Some(num) = size.strip_suffix('B').or_else(|| size.strip_suffix('b')) {
        num.parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    } else {
        size.parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    };

    res.map(|e| e.round() as usize)
}
