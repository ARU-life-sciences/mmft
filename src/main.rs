use anyhow::Result;
use clap::{crate_version, Arg, Command};
use clap::{value_parser, ArgAction};
use std::path::PathBuf;
use std::process;

use mmft::fasta::extract;
use mmft::fasta::filter;
use mmft::fasta::gc;
use mmft::fasta::length;
use mmft::fasta::merge;
use mmft::fasta::min;
use mmft::fasta::n50;
use mmft::fasta::number;
use mmft::fasta::regex;
use mmft::fasta::reverse;
use mmft::fasta::sample;
use mmft::fasta::skew;
use mmft::fasta::split;
use mmft::fasta::stats;
use mmft::fasta::telomere;
use mmft::fasta::translate;

fn main() -> Result<()> {
    let matches = Command::new("mmft")
        .version(crate_version!())
        .author("Max Brown <max.carter-brown@aru.ac.uk>")
        .about("My Minimal Fasta Toolkit")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("len")
                .about("Calculate lengths of fasta file records.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("extract")
                        .long("extract")
                        .short('e')
                        .value_parser(value_parser!(usize))
                        .num_args(1)
                        .required(false)
                        .help("Fasta records with a length greater than specified are printed."),
                )
                .arg(Arg::new("less")
                    .long("less")
                    .action(ArgAction::SetTrue)
                    .short('l').help(
                    "Print records with lengths less than value of extract. Default is greater.",
                )),
        )
        .subcommand(
            Command::new("gc")
                .about("Calculate GC content of fasta file records.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("n50")
                .about("Calculate n50 of fasta files.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("regex")
                .about("Extract fasta records using regex on headers.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .num_args(1)
                        .required(true)
                        .help("Regex to compile."),
                )
                .arg(
                    Arg::new("inverse")
                        .short('i')
                        .long("inverse")
                        .action(ArgAction::SetTrue)
                        .help("Inverse regex match."),
                ),
        )
        .subcommand(
            Command::new("extract")
                .about("Extract (sub)sequence within a fasta file record.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("region")
                        .short('r')
                        .long("region")
                        .num_args(1)
                        .required(true)
                        .help("Numeric region to extract."),
                ),
        )
        .subcommand(
            Command::new("num")
                .about("Calculate number and total base count of fasta file records.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("merge")
                .about(
                    "Merge sequence records within/between fasta files into a single fasta record.",
                )
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("track")
                        .short('t')
                        .long("track")
                        .action(ArgAction::SetTrue)
                        .help("Keep track of record indexes in fasta header")
                )
                .arg(Arg::new("pad")
                        .short('p')
                        .long("pad")
                        .help("Number of N's to insert between merged contigs")
                        .value_parser(clap::value_parser!(usize))
                        .default_value("0"))
                .arg(
                    Arg::new("header")
                        .long("header")
                        .num_args(1)
                        .help("Name of output fasta header."),
                ),
        )
        .subcommand(
            Command::new("trans")
                .about("Translate a fasta into all six frames.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("filter")
                .about("Filter sequences on a file of ID's")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .num_args(1)
                        .required(true)
                        .help("Name of text file with one ID per line."),
                ),
        )
        .subcommand(
            Command::new("sample")
                .about("Randomly sample records from a fasta file.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("sample-number")
                        .short('n')
                        .long("sample-number")
                        .value_parser(value_parser!(i32))
                        .num_args(1)
                        .required_unless_present("sample-size")
                        .help("Number of records to sample."),
                )
                .arg(
                    Arg::new("sample-size")
                        .short('s')
                        .long("sample-size")
                        .value_parser(value_parser!(String))
                        .num_args(1)
                        .required_unless_present("sample-number")
                        .help("Target file size for the sampled sequences (e.g. 10Mb, 3Kb, 5Gb)")
                ),
        )
        .subcommand(
            Command::new("reverse")
                .about("Reverse complement each record in an input fasta")
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
        )
        .subcommand(
            Command::new("min")
                .about("Return the lexicographically minimal rotation of fasta file record sequences.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("split")
                .about("Split a fasta into multiple files based on record count.")
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("number")
                        .short('n')
                        .long("number")
                        .value_parser(value_parser!(i32))
                        .num_args(1)
                        .required(true)
                        .help("Number of records per file."),
                )
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(1)
                        .default_value(".")
                        .help(
                            "Output directory for split files. Default is current working directory.",
                        ),

                )
        )
        .subcommand(
            Command::new("stats")
                .about("Aggregate per-file assembly statistics (n_seqs, total_bp, GC%, N%, N50/L50, auN) in a single pass.")
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("telomere")
                .about("Scan contig ends for canonical telomere repeat motifs.")
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("motif")
                        .short('m')
                        .long("motif")
                        .num_args(1)
                        .required(false)
                        .help("Telomere repeat motif to search for (e.g. TTAGGG). Defaults to trying a built-in set of known eukaryotic motifs."),
                )
                .arg(
                    Arg::new("window")
                        .short('w')
                        .long("window")
                        .value_parser(value_parser!(usize))
                        .num_args(1)
                        .required(false)
                        .help("Number of bases from each end of the contig to search. Default 10000."),
                )
                .arg(
                    Arg::new("min-repeats")
                        .long("min-repeats")
                        .value_parser(value_parser!(usize))
                        .num_args(1)
                        .required(false)
                        .help("Minimum number of consecutive repeat units required to report a match. Default 3."),
                ),
        )
        .subcommand(
            Command::new("skew")
                .about("Calculate GC skew ((G-C)/(G+C)) per window along each sequence, plus its cumulative sum.")
                .arg(
                    Arg::new("fasta")
                        .value_parser(value_parser!(PathBuf))
                        .num_args(0..)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("window")
                        .short('w')
                        .long("window")
                        .value_parser(value_parser!(usize))
                        .num_args(1)
                        .required(false)
                        .help("Window size in bases. Default 1000."),
                ),
        )
        .get_matches();

    // feed command line options to each main function
    match matches.subcommand() {
        Some(("len", matches)) => {
            length::get_lengths(matches)?;
        }
        Some(("gc", matches)) => {
            gc::get_gc(matches)?;
        }
        Some(("n50", matches)) => {
            n50::get_n50(matches)?;
        }
        Some(("regex", matches)) => {
            regex::regex_sequences(matches)?;
        }
        Some(("extract", matches)) => {
            extract::extract_region(matches)?;
        }
        Some(("num", matches)) => {
            number::get_number_seq_bases(matches)?;
        }
        Some(("merge", matches)) => {
            merge::merge_fastas(matches)?;
        }
        Some(("filter", matches)) => {
            filter::filter_sequences(matches)?;
        }
        Some(("trans", matches)) => {
            translate::six_frame_translate(matches)?;
        }
        Some(("sample", matches)) => {
            sample::sample(matches)?;
        }
        Some(("reverse", matches)) => reverse::reverse(matches)?,
        Some(("min", matches)) => {
            min::min(matches)?;
        }
        Some(("split", matches)) => {
            split::split_fasta(matches)?;
        }
        Some(("stats", matches)) => {
            stats::get_stats(matches)?;
        }
        Some(("telomere", matches)) => {
            telomere::find_telomeres(matches)?;
        }
        Some(("skew", matches)) => {
            skew::get_skew(matches)?;
        }
        _ => {
            println!("Subcommand invalid, run with '--help' for subcommand options. Exiting.");
            process::exit(1);
        }
    }
    Ok(())
}
