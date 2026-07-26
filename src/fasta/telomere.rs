use crate::utils::{error, revcomp::reverse_complement, stdin};
use anyhow::{bail, Result};

// Canonical telomeric repeat motifs seen across eukaryotes. Source: widely
// cited consensus repeats used by telomere-identification tools (e.g.
// Sanger's tidk). When the user doesn't supply their own motif with
// `--motif`, each of these (and its reverse complement) is tried at both
// ends of every sequence, and the best-scoring one is reported.
const DEFAULT_MOTIFS: &[(&str, &str)] = &[
    ("vertebrate/metazoan", "TTAGGG"),
    ("plant (Arabidopsis-type)", "TTTAGGG"),
    ("insect", "TTAGG"),
    ("nematode (C. elegans)", "TTAGGC"),
    ("ciliate", "TTGGGG"),
];

pub fn find_telomeres(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let motif = matches.get_one::<String>("motif").map(|s| s.to_uppercase());
    let window = *matches.get_one::<usize>("window").unwrap_or(&10_000);
    let min_repeats = *matches.get_one::<usize>("min-repeats").unwrap_or(&3);

    let candidates: Vec<(String, Vec<u8>)> = match &motif {
        Some(m) => vec![(m.clone(), m.as_bytes().to_vec())],
        None => DEFAULT_MOTIFS
            .iter()
            .map(|(name, m)| (name.to_string(), m.as_bytes().to_vec()))
            .collect(),
    };

    println!("file\tid\tstart_motif\tstart_repeats\tend_motif\tend_repeats");

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    let id = std::str::from_utf8(record.name())?;
                    let seq = record.sequence().as_ref();
                    let report = scan_record(seq, &candidates, window, min_repeats);
                    println!("{}\t{}\t{}", basename, id, report);
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
                    let seq = record.sequence().as_ref();
                    let report = scan_record(seq, &candidates, window, min_repeats);
                    println!("stdin\t{}\t{}", id, report);
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}

fn scan_record(
    seq: &[u8],
    candidates: &[(String, Vec<u8>)],
    window: usize,
    min_repeats: usize,
) -> String {
    let start_window = &seq[..window.min(seq.len())];
    let end_window = &seq[seq.len().saturating_sub(window)..];

    let (start_name, start_count) = best_match(start_window, candidates);
    let (end_name, end_count) = best_match(end_window, candidates);

    let fmt = |name: &str, count: usize| -> String {
        if count >= min_repeats {
            format!("{}\t{}", name, count)
        } else {
            "none\t0".to_string()
        }
    };

    format!("{}\t{}", fmt(&start_name, start_count), fmt(&end_name, end_count))
}

// Tries every candidate motif and its reverse complement against `window`,
// keeping whichever produces the longest run of consecutive tandem repeats.
fn best_match(window: &[u8], candidates: &[(String, Vec<u8>)]) -> (String, usize) {
    let mut best_name = String::new();
    let mut best_count = 0usize;

    for (name, motif) in candidates {
        let rc = reverse_complement(motif);

        let fwd = longest_tandem_run(window, motif);
        if fwd > best_count {
            best_count = fwd;
            best_name = name.clone();
        }

        let rev = longest_tandem_run(window, &rc);
        if rev > best_count {
            best_count = rev;
            best_name = format!("{} (rc)", name);
        }
    }

    (best_name, best_count)
}

// Finds the longest run of consecutive, non-overlapping copies of `motif`
// within `seq`, trying every phase offset (0..motif.len()) since a real
// terminal repeat array rarely starts exactly at index 0.
fn longest_tandem_run(seq: &[u8], motif: &[u8]) -> usize {
    let m = motif.len();
    if m == 0 || seq.len() < m {
        return 0;
    }

    let mut best = 0usize;
    for phase in 0..m {
        let mut run = 0usize;
        let mut i = phase;
        while i + m <= seq.len() {
            if &seq[i..i + m] == motif {
                run += 1;
                best = best.max(run);
            } else {
                run = 0;
            }
            i += m;
        }
    }
    best
}
