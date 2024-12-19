use crate::{
    utils::{error, parse::parse_region, stdin},
    FID,
};
use anyhow::{bail, Result};
use noodles_core::Position;
use noodles_fasta::{
    self as fasta,
    record::{Definition, Sequence},
    Record,
};
use std::io;

pub fn extract_region(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let region = matches
        .get_one::<String>("region")
        .expect("handled by clap");

    let parsed_region = parse_region(region)?;

    // writer here?
    let mut writer = fasta::io::Writer::new(io::stdout());

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;

                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    extract_inner(
                        &record?,
                        parsed_region.clone(),
                        basename.clone(),
                        &mut writer,
                    )?;
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut records = crate::fasta_reader_stdin();

                let mut records = records.records();
                while let Some(Ok(record)) = records.next() {
                    extract_inner(
                        &record,
                        parsed_region.clone(),
                        "stdin".to_string(),
                        &mut writer,
                    )?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}

fn extract_inner(
    record: &Record,
    parsed_region: Vec<usize>,
    basename: String,
    writer: &mut fasta::io::Writer<io::Stdout>,
) -> Result<()> {
    let id = crate::fasta_id_description(&record, FID::Id)?;
    let description = crate::fasta_id_description(&record, FID::Description)?;

    let start = Position::try_from(parsed_region[0])?;
    let end = Position::try_from(parsed_region[1])?;

    let seq = record.sequence().get(start..end);

    let seq_res = match seq {
        Some(s) => s,
        None => bail!(error::RegionError::SeqExtractError),
    };
    // write to stdout
    let description = format!(
        "{}:{}:{}-{}",
        description, basename, parsed_region[0], parsed_region[1]
    );

    let definition = Definition::new(id, Some(description.into_bytes()));

    let record = fasta::Record::new(definition, Sequence::from(seq_res.to_vec()));

    let _ = writer
        .write_record(&record)
        .map_err(|_| error::FastaWriteError::CouldNotWrite);

    Ok(())
}
