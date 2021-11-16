use crate::utils::{error, stdin};
use anyhow::{bail, Result};
use bio::io::fasta;
use std::io;

pub fn merge_fastas(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.values_of("fasta");
    let headers_op = matches.value_of("header");

    match input_file {
        // read directly from files
        Some(f) => {
            // print header
            match headers_op {
                Some(h) => println!(">{}", h),
                None => println!(">merged"),
            }
            for el in f {
                let reader = fasta::Reader::from_file(el).expect("[-]\tPath invalid.");
                for record in reader.records() {
                    let record = record.expect("[-]\tError during fasta record parsing.");
                    let seq = std::str::from_utf8(record.seq());
                    // write to stdout
                    match seq {
                        Ok(s) => print!("{}", s),
                        Err(_) => bail!(error::UTF8FormatError::NotUTF8),
                    }
                }
            }
            print!("\n");
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                // print header
                match headers_op {
                    Some(h) => println!(">{}", h),
                    None => println!(">merged"),
                }
                let mut records = fasta::Reader::new(io::stdin()).records();
                while let Some(Ok(record)) = records.next() {
                    let seq = std::str::from_utf8(record.seq());
                    // write to stdout
                    match seq {
                        Ok(s) => print!("{}", s),
                        Err(_) => bail!(error::UTF8FormatError::NotUTF8),
                    }
                }
                print!("\n");
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
