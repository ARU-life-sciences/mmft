use anyhow::Result;
use clap::{crate_version, Arg, Command};
use std::process;

use mmft::fasta::extract;
use mmft::fasta::filter;
use mmft::fasta::gc;
use mmft::fasta::length;
use mmft::fasta::merge;
use mmft::fasta::n50;
use mmft::fasta::number;
use mmft::fasta::regex;
use mmft::fasta::sample;
use mmft::fasta::translate;

fn main() -> Result<()> {
    let matches = Command::new("mmft")
        .version(crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("My Minimal Fasta Toolkit")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("len")
                .about("Calculate lengths of fasta file records.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_values(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("extract")
                        .long("extract")
                        .short('e')
                        .takes_value(true)
                        .required(false)
                        .help("Fasta records with a length greater than specified are printed."),
                )
                .arg(Arg::new("less").long("less").short('l').help(
                    "Print records with lengths less than value of extract. Default is greater.",
                )),
        )
        .subcommand(
            Command::new("gc")
                .about("Calculate GC content of fasta file records.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_values(true)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("n50")
                .about("Calculate n50 of fasta files.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("regex")
                .about("Extract fasta records using regex on headers.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .takes_value(true)
                        .help("Regex to compile."),
                )
                .arg(
                    Arg::new("inverse")
                        .short('i')
                        .long("inverse")
                        .help("Inverse regex match."),
                ),
        )
        .subcommand(
            Command::new("extract")
                .about("Extract (sub)sequence within a fasta file record.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("region")
                        .short('r')
                        .long("region")
                        .takes_value(true)
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
                        .multiple_occurrences(true)
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
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("header")
                        .long("header")
                        .takes_value(true)
                        .help("Name of output fasta header."),
                ),
        )
        .subcommand(
            Command::new("trans")
                .about("Translate a fasta into all six frames.")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            Command::new("filter")
                .about("Filter sequences on a file of ID's")
                // output file name
                .arg(
                    Arg::new("fasta")
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .takes_value(true)
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
                        .multiple_occurrences(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::new("sample-number")
                        .short('n')
                        .long("sample-number")
                        .takes_value(true)
                        .required(true)
                        .help("Number of records to sample."),
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
        _ => {
            println!("Subcommand invalid, run with '--help' for subcommand options. Exiting.");
            process::exit(1);
        }
    }
    Ok(())
}
