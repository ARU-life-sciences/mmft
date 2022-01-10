use anyhow::Result;
use clap::{App, Arg};
use std::process;

use mmft::fasta::extract;
use mmft::fasta::filter;
use mmft::fasta::gc;
use mmft::fasta::length;
use mmft::fasta::merge;
use mmft::fasta::n50;
use mmft::fasta::number;
use mmft::fasta::regex;

fn main() -> Result<()> {
    let matches = App::new("mmft")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("My Minimal Fasta Toolkit")
        .subcommand(
            clap::SubCommand::with_name("len")
                .about("Calculate lengths of fasta file records.")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::with_name("extract")
                        .long("extract")
                        .short("e")
                        .takes_value(true)
                        .required(false)
                        .help("Fasta records with a length greater than specified are printed."),
                )
                .arg(Arg::with_name("less").long("less").short("l").help(
                    "Print records with lengths less than value of extract. Default is greater.",
                )),
        )
        .subcommand(
            clap::SubCommand::with_name("gc")
                .about("Calculate GC content of fasta file records.")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("n50")
                .about("Calculate n50 of fasta files.")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("regex")
                .about("Extract fasta records using regex on headers.")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::with_name("regex")
                        .short("r")
                        .long("regex")
                        .takes_value(true)
                        .help("Regex to compile."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("extract")
                .about("Extract (sub)sequence within a fasta file record.")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::with_name("region")
                        .short("r")
                        .long("region")
                        .takes_value(true)
                        .required(true)
                        .help("Numeric region to extract."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("num")
                .about("Calculate number and total base count of fasta file records.")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("merge")
                .about(
                    "Merge sequence records within/between fasta files into a single fasta record.",
                )
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::with_name("header")
                        .short("h")
                        .long("header")
                        .takes_value(true)
                        .help("Name of output fasta header."),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("filter")
                .about("Filter sequences on a file of ID's")
                // output file name
                .arg(
                    Arg::with_name("fasta")
                        .multiple(true)
                        .help("Input fasta file path(s)."),
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .help("Name of text file with one ID per line."),
                ),
        )
        .get_matches();

    // feed command line options to each main function
    let subcommand = matches.subcommand();
    match subcommand.0 {
        "len" => {
            let matches = subcommand.1.unwrap();
            length::get_lengths(matches)?;
        }
        "gc" => {
            let matches = subcommand.1.unwrap();
            gc::get_gc(matches)?;
        }
        "n50" => {
            let matches = subcommand.1.unwrap();
            n50::get_n50(matches)?;
        }
        "regex" => {
            let matches = subcommand.1.unwrap();
            regex::regex_sequences(matches)?;
        }
        "extract" => {
            let matches = subcommand.1.unwrap();
            extract::extract_region(matches)?;
        }
        "num" => {
            let matches = subcommand.1.unwrap();
            number::get_number_seq_bases(matches)?;
        }
        "merge" => {
            let matches = subcommand.1.unwrap();
            merge::merge_fastas(matches)?;
        }
        "filter" => {
            let matches = subcommand.1.unwrap();
            filter::filter_sequences(matches)?;
        }
        _ => {
            println!("Subcommand invalid, run with '--help' for subcommand options. Exiting.");
            process::exit(1);
        }
    }
    Ok(())
}
