use crate::FID;
use anyhow::{bail, Result};
use noodles_fasta as fasta;
use rand::Rng;
use std::io;

pub fn sample(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let sample_number = matches.get_one::<i32>("sample-number").cloned();
    let sample_size = matches
        .get_one::<String>("sample-size")
        .map(|s| parse_size(s))
        .transpose()?;

    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        Some(f) => {
            for el in f {
                if let Some(sn) = sample_number {
                    let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                    let mut total_records = 0;
                    for result in reader.records() {
                        let _ = result?;
                        total_records += 1;
                    }

                    if sn > total_records {
                        bail!("Sample number is greater than total records.");
                    }

                    let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                    let mut remaining_to_sample = sn;
                    let mut remaining_in_file = total_records;

                    let mut rng = rand::thread_rng();

                    for result in reader.records() {
                        let record = result?;

                        let prob = remaining_to_sample as f64 / remaining_in_file as f64;
                        if rng.gen_bool(prob) {
                            // Write the record if selected
                            writer.write_record(&record)?;
                            remaining_to_sample -= 1;

                            // Stop early if we've sampled enough
                            if remaining_to_sample == 0 {
                                break;
                            }
                        }
                        remaining_in_file -= 1;
                    }
                }

                // else do sample size
                if let Some(ss) = sample_size {
                    let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                    let mut total_records = 0;
                    for result in reader.records() {
                        let _ = result?;
                        total_records += 1;
                    }

                    let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                    let mut rng = rand::thread_rng();
                    let mut total_bytes_written: usize = 0;
                    let mut remaining_records = total_records;

                    for result in reader.records() {
                        let record = result?;

                        // Approximate the size of the current record in bytes
                        let record_bytes =
                            crate::fasta_id_description(&record, FID::Both("".into()))?.len()
                                + record.sequence().len()
                                + 2; // ID + sequence + newlines

                        // Calculate dynamic sampling probability
                        let prob = (ss - total_bytes_written) as f64
                            / (record_bytes as f64 * remaining_records as f64);

                        // Randomly decide whether to include this record
                        if rng.gen_bool(prob.clamp(0.0, 1.0)) {
                            if total_bytes_written + record_bytes > ss {
                                break; // Stop if adding this record would exceed the byte limit
                            }

                            writer.write_record(&record)?;
                            total_bytes_written += record_bytes;
                        }

                        remaining_records -= 1; // Decrease the number of records remaining to process
                    }
                }
            }
        }
        None => bail!("STDIN not supported for this command."),
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
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    } else if let Some(num) = size.strip_suffix('B').or_else(|| size.strip_suffix('b')) {
        num.parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    } else {
        size.parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid size format"))
    };

    res.map(|e| e.round() as usize)
}
