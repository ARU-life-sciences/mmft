use std::io::{self, Write};

/// Streams raw sequence bytes to an inner writer, hard-wrapping lines at
/// `width` bases (matching noodles' own default FASTA line width) without
/// ever buffering more than one line's worth of sequence in memory. This
/// lets merge concatenate arbitrarily large genomes into standard-looking
/// wrapped FASTA while still writing incrementally as records are read.
pub struct LineWrapWriter<W: Write> {
    inner: W,
    width: usize,
    col: usize,
}

impl<W: Write> LineWrapWriter<W> {
    pub fn new(inner: W, width: usize) -> Self {
        Self {
            inner,
            width,
            col: 0,
        }
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Writes a chunk of sequence, inserting newlines every `width` bases
    /// regardless of chunk boundaries.
    pub fn write_seq(&mut self, mut seq: &[u8]) -> io::Result<()> {
        while !seq.is_empty() {
            let remaining_in_line = self.width - self.col;
            let take = remaining_in_line.min(seq.len());

            self.inner.write_all(&seq[..take])?;
            self.col += take;
            seq = &seq[take..];

            if self.col == self.width {
                self.inner.write_all(b"\n")?;
                self.col = 0;
            }
        }
        Ok(())
    }

    /// Terminates the current line if it was left partially filled.
    pub fn finish(&mut self) -> io::Result<()> {
        if self.col > 0 {
            self.inner.write_all(b"\n")?;
            self.col = 0;
        }
        Ok(())
    }
}
