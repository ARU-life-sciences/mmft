use crate::utils::{error, stdin};
use anyhow::{bail, Result};

pub fn merge_fastas(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = crate::get_fasta_files(matches);
    let headers_op = matches.get_one::<String>("header");
    let track = matches.get_flag("track");

    let mut seq = String::new();
    let mut tracking_header = String::new();
    let mut cum_seq_len = 1;

    match input_file {
        // read directly from files
        Some(f) => {
            // print header
            // if we are not using tracking info
            if !track {
                match headers_op {
                    Some(h) => println!(">{}", h),
                    None => println!(">merged"),
                }
            }
            for el in f {
                let mut reader = crate::fasta_reader_file(el.to_path_buf())?;
                for record in reader.records() {
                    let record = record?;
                    // write to stdout
                    if !track {
                        print!("{}", std::str::from_utf8(record.sequence().as_ref())?);
                    } else {
                        let seq_len = cum_seq_len + record.sequence().len() - 1;
                        let rec_name = std::str::from_utf8(record.name())?;
                        tracking_header += &format!("{rec_name},{cum_seq_len}-{seq_len}:");

                        seq += std::str::from_utf8(record.sequence().as_ref())?;
                        cum_seq_len += record.sequence().len();
                    }
                }
            }
            if !track {
                println!();
            } else {
                // remove last :
                tracking_header.pop();
                println!(">{}", tracking_header);
                println!("{}", seq);
            }
        }
        // read from stdin
        None => match stdin::is_stdin() {
            true => {
                // print header
                if !track {
                    match headers_op {
                        Some(h) => println!(">{}", h),
                        None => println!(">merged"),
                    }
                }
                let mut reader = crate::fasta_reader_stdin();
                let mut records = reader.records();
                while let Some(Ok(record)) = records.next() {
                    if !track {
                        print!("{}", std::str::from_utf8(record.sequence().as_ref())?);
                    } else {
                        let seq_len = cum_seq_len + record.sequence().len() - 1;
                        let rec_name = std::str::from_utf8(record.name())?;
                        tracking_header += &format!("{rec_name},{cum_seq_len}-{seq_len}:");

                        seq += std::str::from_utf8(record.sequence().as_ref())?;
                        cum_seq_len += record.sequence().len();
                    }
                }
                if !track {
                    println!();
                } else {
                    // remove last :
                    tracking_header.pop();
                    println!(">{}", tracking_header);
                    println!("{}", seq);
                }
            }
            false => {
                bail!(error::StdinError::NoSequence)
            }
        },
    }
    Ok(())
}
