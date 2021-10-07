use anyhow::Result;
use clap::{App, Arg};
use std::process;

use mmft::fasta::gc;
use mmft::fasta::length;
use mmft::fasta::n50;

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
                ),
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
        _ => {
            println!("Subcommand invalid, run with '--help' for subcommand options. Exiting.");
            process::exit(1);
        }
    }
    Ok(())
}
