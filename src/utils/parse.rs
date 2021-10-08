// parse a str like this
// 1000 - 10000
// into usize

use crate::utils::error;
use anyhow::{bail, Result};

pub fn parse_region(region: &str) -> Result<Vec<usize>> {
    let str_rmw = &remove_whitespace(region);

    let split_vec: Result<Vec<usize>, _> = str_rmw.split('-').map(|x| x.parse::<usize>()).collect();

    let res = match split_vec {
        Ok(s) => s,
        Err(e) => {
            let format_err = format!("{}\nError: [-]\t{}", error::RegionError::CouldNotParse, e);
            bail!(format_err);
        }
    };

    if res.len() != 2 {
        bail!("[-]\tPlease input a range between two numbers only.");
    }

    Ok(res)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
