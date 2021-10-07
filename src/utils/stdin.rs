use atty::Stream;

// check if there is anything coming from stdin
// (this was a stranger solution than I had anticipated)
pub fn is_stdin() -> bool {
    !atty::is(Stream::Stdin)
}
