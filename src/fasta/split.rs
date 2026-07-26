use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use noodles_fasta as fasta;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn split_fasta(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let split_number = *matches.get_one::<i32>("number").expect("errored by clap");
    let dir = matches
        .get_one::<PathBuf>("dir")
        .cloned()
        .expect("defaulted by clap");

    if split_number <= 0 {
        bail!("Number of records per file must be greater than zero.");
    }
    let split_number = split_number as usize;

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;
                // ugly but should work
                let basename = basename
                    .strip_suffix(".fasta")
                    .unwrap_or(&basename)
                    .strip_suffix(".fa")
                    .unwrap_or(&basename);

                // single pass: roll over to a new chunk file every
                // `split_number` records, instead of reading the whole file
                // once to count records and then again to write them out
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                let mut chunk_index = 0usize;
                let mut count_in_chunk = 0usize;
                let mut writer: Option<fasta::Writer<BufWriter<fs::File>>> = None;

                for record in reader.records() {
                    let record = record?;

                    if writer.is_none() {
                        let chunk_file = fs::File::create(
                            dir.join(format!("{}_chunk_{}.fa", basename, chunk_index)),
                        )?;
                        writer = Some(fasta::Writer::new(BufWriter::new(chunk_file)));
                    }

                    writer.as_mut().unwrap().write_record(&record)?;
                    count_in_chunk += 1;

                    if count_in_chunk == split_number {
                        writer.take().unwrap().get_mut().flush()?;
                        chunk_index += 1;
                        count_in_chunk = 0;
                    }
                }

                if let Some(mut w) = writer {
                    w.get_mut().flush()?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                todo!()
                // let mut records = fasta::Reader::new(io::stdin()).records();
                // while let Some(Ok(record)) = records.next() {}
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}
