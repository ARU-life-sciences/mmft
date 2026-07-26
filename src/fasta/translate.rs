use crate::utils::{error, revcomp, stdin, translate};
use anyhow::{bail, Result};
use noodles_fasta::record::Definition;
use noodles_fasta::{self as fasta, Record};
use std::fmt;
use std::io::{self, BufWriter, Write};

#[derive(Clone, Copy)]
enum Orientation {
    Forward,
    Reverse,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Orientation::Forward => write!(f, "forward"),
            Orientation::Reverse => write!(f, "reverse"),
        }
    }
}

pub fn six_frame_translate(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    let mut writer = fasta::Writer::new(BufWriter::new(io::stdout()));

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    translate_all_frames(&record?, &mut writer)?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    translate_all_frames(&record, &mut writer)?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    writer.get_mut().flush()?;
    Ok(())
}

// Translates all six frames (3 forward skips, 3 reverse skips) for a single
// record. The reverse complement of the sequence is computed once here and
// reused for all three reverse-strand skips, rather than being recomputed
// from scratch for each one.
fn translate_all_frames<W: Write>(r: &Record, writer: &mut fasta::Writer<W>) -> Result<()> {
    let forward = r.sequence().as_ref();
    let reverse = revcomp::reverse_complement(forward);

    for skip in 0..=2 {
        translate_inner(Orientation::Forward, skip, r, &forward[skip..], writer)?;
    }
    for skip in 0..=2 {
        translate_inner(Orientation::Reverse, skip, r, &reverse[skip..], writer)?;
    }

    Ok(())
}

fn translate_inner<W: Write>(
    strand: Orientation,
    skip: usize,
    r: &Record,
    seq: &[u8],
    writer: &mut fasta::Writer<W>,
) -> Result<()> {
    let definition = Definition::new(
        r.name(),
        r.description().map(|s| {
            // own and add to s
            let mut s = s.to_vec();
            s.append(
                &mut format!(":strand={}:skip={}", strand, skip)
                    .as_bytes()
                    .to_vec(),
            );
            s
        }),
    );

    let record = Record::new(definition, translate::translate(seq).into());

    writer
        .write_record(&record)
        .map_err(|_| error::FastaWriteError::CouldNotWrite)?;

    Ok(())
}
