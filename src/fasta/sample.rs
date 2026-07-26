use crate::FID;
use anyhow::{bail, Result};
use noodles_fasta::{self as fasta, Record};
use rand::Rng;
use std::io::{self, BufWriter, Write};

pub fn sample(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let sample_number = matches.get_one::<i32>("sample-number").cloned();
    let sample_size = matches
        .get_one::<String>("sample-size")
        .map(|s| parse_size(s))
        .transpose()?;

    let mut writer = fasta::Writer::new(BufWriter::new(io::stdout()));

    match input_file {
        Some(f) => {
            for el in f {
                if let Some(sn) = sample_number {
                    if sn <= 0 {
                        bail!("Sample number must be greater than zero.");
                    }
                    let sn = sn as usize;

                    // Single-pass reservoir sampling (Algorithm R): avoids
                    // reading and parsing the file twice just to learn the
                    // total record count up front. Each record's original
                    // index is kept alongside it so the sample can be
                    // written back out in file order afterwards.
                    let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                    let mut rng = rand::rng();
                    let mut reservoir: Vec<(usize, Record)> = Vec::with_capacity(sn);

                    for (i, result) in reader.records().enumerate() {
                        let record = result?;
                        if i < sn {
                            reservoir.push((i, record));
                        } else {
                            let j = rng.random_range(0..=i);
                            if j < sn {
                                reservoir[j] = (i, record);
                            }
                        }
                    }

                    if reservoir.len() < sn {
                        bail!("Sample number is greater than total records.");
                    }

                    reservoir.sort_unstable_by_key(|(idx, _)| *idx);
                    for (_, record) in reservoir {
                        writer.write_record(&record)?;
                    }
                }

                // else do sample size
                if let Some(ss) = sample_size {
                    // Read the file once and keep the records in memory,
                    // rather than reopening and reparsing it just to count
                    // records before the sampling pass.
                    let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                    let records: Vec<Record> = reader.records().collect::<io::Result<_>>()?;
                    let total_records = records.len();

                    let mut rng = rand::rng();
                    let mut total_bytes_written: usize = 0;
                    let mut remaining_records = total_records;

                    for record in records {
                        // Approximate the size of the current record in bytes
                        let record_bytes =
                            crate::fasta_id_description(&record, FID::Both("".into()))?.len()
                                + record.sequence().len()
                                + 2; // ID + sequence + newlines

                        // Calculate dynamic sampling probability
                        let prob = (ss - total_bytes_written) as f64
                            / (record_bytes as f64 * remaining_records as f64);

                        // Randomly decide whether to include this record
                        if rng.random_bool(prob.clamp(0.0, 1.0)) {
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
    writer.get_mut().flush()?;
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
