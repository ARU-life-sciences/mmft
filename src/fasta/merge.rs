use crate::utils::{error, stdin, wrap::LineWrapWriter};
use anyhow::{bail, Result};
use std::io::{self, BufWriter, Write};

// matches noodles' own default FASTA line width, so merge's output looks
// like every other subcommand's fasta output instead of one giant unwrapped
// line
const LINE_WIDTH: usize = 80;

pub fn merge_fastas(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let headers_op = matches.get_one::<String>("header");
    let track = matches.get_flag("track");

    let pad_length = *matches.get_one::<usize>("pad").unwrap_or(&0);
    let pad_string = "N".repeat(pad_length);
    let header = headers_op.map(String::as_str).unwrap_or("merged");

    let mut writer = LineWrapWriter::new(BufWriter::new(io::stdout()), LINE_WIDTH);
    writeln!(writer.get_mut(), ">{}", header)?;

    let mut contig_positions: Vec<(String, usize)> = Vec::new();
    let mut cum_seq_len: usize = 1; // 1-based coordinate tracking
    let mut first_seq = true;

    match input_file {
        Some(files) => {
            for el in files {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;

                for record in reader.records() {
                    let record = record?;
                    let rec_name = std::str::from_utf8(record.name())?.to_string();

                    write_merged_record(
                        &mut writer,
                        &mut contig_positions,
                        &mut cum_seq_len,
                        &mut first_seq,
                        track,
                        pad_length,
                        &pad_string,
                        rec_name,
                        record.sequence().as_ref(),
                    )?;
                }
            }
        }
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();

                while let Some(Ok(record)) = records.next() {
                    let rec_name = std::str::from_utf8(record.name())?.to_string();

                    write_merged_record(
                        &mut writer,
                        &mut contig_positions,
                        &mut cum_seq_len,
                        &mut first_seq,
                        track,
                        pad_length,
                        &pad_string,
                        rec_name,
                        record.sequence().as_ref(),
                    )?;
                }
            }
            false => {
                bail!(error::StdinError::NoSequence);
            }
        },
    }

    writer.finish()?;
    writer.get_mut().flush()?;

    if track {
        let mut stderr = io::stderr();
        for (contig, start) in &contig_positions {
            writeln!(stderr, "{}\t{}", contig, start)?;
        }
    }

    Ok(())
}

// Streams a single record's sequence straight to the wrapped writer instead
// of accumulating the whole merged sequence in a String first: for
// multi-gigabase merges this keeps peak memory to the size of
// `contig_positions` rather than the size of the merged genome.
#[allow(clippy::too_many_arguments)]
fn write_merged_record(
    writer: &mut LineWrapWriter<BufWriter<io::Stdout>>,
    contig_positions: &mut Vec<(String, usize)>,
    cum_seq_len: &mut usize,
    first_seq: &mut bool,
    track: bool,
    pad_length: usize,
    pad_string: &str,
    rec_name: String,
    seq: &[u8],
) -> Result<()> {
    if !*first_seq && pad_length > 0 {
        writer.write_seq(pad_string.as_bytes())?;
        *cum_seq_len += pad_length;
    }

    if track {
        contig_positions.push((rec_name, *cum_seq_len));
    }

    writer.write_seq(seq)?;
    *cum_seq_len += seq.len();
    *first_seq = false;

    Ok(())
}
