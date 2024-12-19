use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use noodles_fasta as fasta;
use std::fs;
use std::path::PathBuf;

pub fn split_fasta(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let split_number = *matches.get_one::<i32>("number").expect("errored by clap");
    let dir = matches
        .get_one::<PathBuf>("dir")
        .cloned()
        .expect("defaulted by clap");

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

                // have to iterate over the file first to get the total number of reads
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                let mut records = reader.records();
                let mut nb_reads = 0;
                while let Some(Ok(_)) = records.next() {
                    nb_reads += 1;
                }
                drop(records);
                // now I want to split the file in a number of files where each file
                // has at most `split_number` reads
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                let mut records = reader.records();

                let chunk_number = (nb_reads as f64 / split_number as f64).floor() as usize;
                let mut chunk_iter: Vec<i32> = (0..chunk_number).map(|_| split_number).collect();
                if nb_reads % split_number != 0 {
                    chunk_iter.push(nb_reads - (split_number * chunk_number as i32));
                }

                // now we can iterate over the chunks, create a new fasta writer in each
                // iteration, and write the number of records specified by the chunk_iter to it
                for (i, chunk) in chunk_iter.iter().enumerate() {
                    let chunk_file =
                        fs::File::create(dir.join(format!("{}_chunk_{}.fa", basename, i)))?;
                    let mut writer = fasta::Writer::new(chunk_file);
                    for _ in 0..*chunk {
                        if let Some(Ok(record)) = records.next() {
                            writer.write_record(&record)?;
                        }
                    }
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
