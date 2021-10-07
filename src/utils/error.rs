use thiserror::Error;

// custom error for no STDIN found.
#[derive(Error, Debug)]
pub enum StdinError {
    #[error("STDIN did not contain any sequence(s).")]
    NoSequence,
}
