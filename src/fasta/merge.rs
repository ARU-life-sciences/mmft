use crate::utils::{error, stdin};
use anyhow::{bail, Result};

pub fn merge_fastas(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let headers_op = matches.get_one::<String>("header");

    match input_file {
        // read directly from files
        Some(f) => {
            // print header
            match headers_op {
                Some(h) => println!(">{}", h),
                None => println!(">merged"),
            }
            for el in f {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    // write to stdout
                    print!("{}", std::str::from_utf8(record.sequence().as_ref())?);
                }
            }
            println!();
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                // print header
                match headers_op {
                    Some(h) => println!(">{}", h),
                    None => println!(">merged"),
                }
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    print!("{}", std::str::from_utf8(record.sequence().as_ref())?);
                }
                println!();
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
