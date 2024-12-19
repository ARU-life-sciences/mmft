use crate::utils::{error, revcomp, stdin, translate};
use anyhow::{bail, Result};
use noodles_fasta::record::Definition;
use noodles_fasta::{self as fasta, Record};
use std::fmt;
use std::io;

#[derive(Clone)]
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

    let mut writer = fasta::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    // now we iterate over forward/reverse
                    let r = record?.clone();
                    for strand in [Orientation::Forward, Orientation::Reverse] {
                        for skip in 0..=2 {
                            translate_inner(strand.clone(), skip, &r, &mut writer)?;
                        }
                    }
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    // now we iterate over forward/reverse
                    let r = record.clone();
                    for strand in [Orientation::Forward, Orientation::Reverse] {
                        for skip in 0..=2 {
                            translate_inner(strand.clone(), skip, &r, &mut writer)?;
                        }
                    }
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }
    Ok(())
}

fn translate_inner(
    strand: Orientation,
    skip: usize,
    r: &Record,
    writer: &mut fasta::Writer<io::Stdout>,
) -> Result<()> {
    // skip the appropriate amount in the sequence
    let seq = match strand {
        Orientation::Forward => &r.sequence().as_ref()[skip..],
        Orientation::Reverse => &revcomp::reverse_complement(r.sequence().as_ref())[skip..],
    };

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

    let _ = writer
        .write_record(&record)
        .map_err(|_| error::FastaWriteError::CouldNotWrite);

    Ok(())
}
