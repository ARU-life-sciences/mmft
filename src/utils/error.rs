use thiserror::Error;

// custom error for no STDIN found.
#[derive(Error, Debug)]
pub enum StdinError {
    #[error("[-]\tSTDIN did not contain any sequence(s).")]
    NoSequence,
}

#[derive(Error, Debug)]
pub enum RegexError {
    #[error("[-]\tCould not compile regex. See https://docs.rs/regex/1.5.4/regex/index.html for examples.")]
    CouldNotCompile,
}

#[derive(Error, Debug)]
pub enum FastaWriteError {
    #[error("[-]\tCould not write to file.")]
    CouldNotWrite,
}
