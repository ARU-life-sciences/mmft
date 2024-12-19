use crate::{
    utils::{error, stdin},
    FID,
};
use anyhow::{bail, Result};
use std::borrow::Borrow;

pub fn get_gc(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    let id = crate::fasta_id_description(&record, FID::Id)?;
                    let description = crate::fasta_id_description(&record, FID::Description)?;
                    let gc = gc_content(record.sequence().as_ref());
                    // write to stdout
                    println!("{}\t{}\t{}\t{}", basename, id, description, gc);
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();

                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    let id = String::from_utf8(record.name().to_vec())?;
                    let description =
                        String::from_utf8(record.description().unwrap_or(&[]).to_vec())?;
                    let gc = gc_content(record.sequence().as_ref());
                    // write to stdout
                    println!("{}\t{}\t{}", id, description, gc);
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}

// see https://github.com/rust-bio/rust-bio/blob/master/src/seq_analysis/gc.rs
fn gc_content<C: Borrow<u8>, T: IntoIterator<Item = C>>(sequence: T) -> f32 {
    let (l, count) = sequence
        .into_iter()
        .fold((0usize, 0usize), |(l, count), n| match *n.borrow() {
            b'c' | b'g' | b'G' | b'C' => (l + 1, count + 1),
            _ => (l + 1, count),
        });
    count as f32 / l as f32
}
