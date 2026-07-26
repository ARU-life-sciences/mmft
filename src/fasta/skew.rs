use crate::utils::{error, stdin};
use anyhow::{bail, Result};

pub fn get_skew(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let window = *matches.get_one::<usize>("window").unwrap_or(&1000);

    if window == 0 {
        bail!("Window size must be greater than zero.");
    }

    println!("file\tid\tstart\tend\tgc_skew\tcumulative_skew");

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    let id = std::str::from_utf8(record.name())?;
                    print_skew(&basename, id, record.sequence().as_ref(), window);
                }
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    let id = std::str::from_utf8(record.name())?;
                    print_skew("stdin", id, record.sequence().as_ref(), window);
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}

// GC skew, (G-C)/(G+C), computed per non-overlapping window along the
// sequence, plus its running cumulative sum. In circular genomes (the
// mitogenome case this was written for) the cumulative curve's minimum and
// maximum tend to mark the origin and terminus of replication, since
// leading- and lagging-strand synthesis have different mutational biases.
fn print_skew(basename: &str, id: &str, seq: &[u8], window: usize) {
    let mut cumulative = 0f64;

    for (chunk_index, chunk) in seq.chunks(window).enumerate() {
        let mut g = 0usize;
        let mut c = 0usize;
        for &b in chunk {
            match b {
                b'G' | b'g' => g += 1,
                b'C' | b'c' => c += 1,
                _ => {}
            }
        }

        let skew = if g + c > 0 {
            (g as f64 - c as f64) / (g + c) as f64
        } else {
            0.0
        };
        cumulative += skew;

        let start = chunk_index * window + 1; // 1-based, inclusive
        let end = start + chunk.len() - 1;
        println!(
            "{}\t{}\t{}\t{}\t{:.4}\t{:.4}",
            basename, id, start, end, skew, cumulative
        );
    }
}
