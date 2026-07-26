use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use noodles_fasta::io::Reader;
use std::io::BufRead;

pub fn get_stats(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);

    println!("file\tn_seqs\ttotal_bp\tgc_pct\tn_pct\tmin_len\tmax_len\tn50\tl50\tauN");

    match input_file {
        // read directly from files
        Some(f) => {
            for el in f.iter() {
                let basename = crate::get_basename_from_pathbuf(el)?;
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                let stats = compute_stats(&mut reader)?;
                println!("{}\t{}", basename, stats);
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                let mut reader = crate::fasta_reader_stdin();
                let stats = compute_stats(&mut reader)?;
                println!("stdin\t{}", stats);
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}

struct Stats {
    n_seqs: usize,
    total_bp: usize,
    gc_pct: f64,
    n_pct: f64,
    min_len: usize,
    max_len: usize,
    n50: usize,
    l50: usize,
    au_n: f64,
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{:.2}\t{:.2}\t{}\t{}\t{}\t{}\t{:.1}",
            self.n_seqs,
            self.total_bp,
            self.gc_pct,
            self.n_pct,
            self.min_len,
            self.max_len,
            self.n50,
            self.l50,
            self.au_n
        )
    }
}

// Single pass over the records: lengths, GC count and N (ambiguous base)
// count are all accumulated together instead of re-reading the file once
// per metric (as running `len`, `gc`, `n50` and `num` separately would).
fn compute_stats<R: BufRead>(reader: &mut Reader<R>) -> Result<Stats> {
    let mut lengths = Vec::new();
    let mut gc_count = 0usize;
    let mut n_count = 0usize;
    let mut total_bp = 0usize;

    for record in reader.records() {
        let record = record?;
        let seq = record.sequence().as_ref();
        total_bp += seq.len();
        lengths.push(seq.len());

        for &b in seq {
            match b {
                b'G' | b'g' | b'C' | b'c' => gc_count += 1,
                b'N' | b'n' => n_count += 1,
                _ => {}
            }
        }
    }

    let n_seqs = lengths.len();
    let min_len = lengths.iter().copied().min().unwrap_or(0);
    let max_len = lengths.iter().copied().max().unwrap_or(0);

    lengths.sort_unstable();
    let (n50, l50) = n50_l50(&lengths, total_bp);

    // auN: area under the Nx curve, sum(l_i^2) / total_length. A single
    // contiguity metric that (unlike N50) varies smoothly as assemblies
    // change, rather than jumping discretely between contig boundaries.
    // See https://lh3.github.io/2020/04/08/a-new-metric-on-assembly-contiguity
    let sum_sq: u128 = lengths.iter().map(|&l| (l as u128) * (l as u128)).sum();
    let au_n = if total_bp > 0 {
        sum_sq as f64 / total_bp as f64
    } else {
        0.0
    };

    let gc_pct = if total_bp > 0 {
        gc_count as f64 / total_bp as f64 * 100.0
    } else {
        0.0
    };
    let n_pct = if total_bp > 0 {
        n_count as f64 / total_bp as f64 * 100.0
    } else {
        0.0
    };

    Ok(Stats {
        n_seqs,
        total_bp,
        gc_pct,
        n_pct,
        min_len,
        max_len,
        n50,
        l50,
        au_n,
    })
}

// `lengths_asc` must be sorted ascending. Returns (N50, L50): the length of
// the shortest contig in the minimal set of longest contigs whose lengths
// sum to at least half the assembly, and the size of that set.
fn n50_l50(lengths_asc: &[usize], total: usize) -> (usize, usize) {
    if lengths_asc.is_empty() {
        return (0, 0);
    }

    let half = total / 2;
    let mut acc = 0usize;
    for (i, &l) in lengths_asc.iter().enumerate() {
        acc += l;
        if acc > half {
            return (l, lengths_asc.len() - i);
        }
    }

    (lengths_asc[lengths_asc.len() - 1], 1)
}
