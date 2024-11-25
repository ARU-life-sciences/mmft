use super::revcomp::reverse_complement;
use anyhow::Result;
use lexical_sort::{natural_lexical_cmp, StringSort};
use std::cmp::min;

/// Booth's algorithm for the lexicographically minimal
/// rotation of a string. Should give us a canonical rotation
/// given a rotated string.
/// Taken from [here](https://github.com/zimpha/algorithmic-library/blob/61e897983314033615bcd278d22a754bfc3c3f22/rust/src/strings/mod.rs)
fn minimal_rotation<T: Ord>(s: &[T]) -> usize {
    let n = s.len();
    let mut i = 0;
    let mut j = 1;
    loop {
        let mut k = 0;
        let mut ci = &s[i % n];
        let mut cj = &s[j % n];
        while k < n {
            ci = &s[(i + k) % n];
            cj = &s[(j + k) % n];
            if ci != cj {
                break;
            }
            k += 1
        }
        if k == n {
            return min(i, j);
        }
        if ci > cj {
            i += k + 1;
            i += (i == j) as usize;
        } else {
            j += k + 1;
            j += (i == j) as usize;
        }
    }
}

pub fn lex_min(dna_string: &[u8]) -> Result<String> {
    // revcomp
    let dna_string_r = reverse_complement(dna_string);
    // index for forward and reverse
    let index_f = minimal_rotation(dna_string);
    let index_r = minimal_rotation(&dna_string_r);
    // create the substrings
    // starts
    let start_f = &dna_string[index_f..];
    let start_r = &dna_string_r[index_r..];
    // ends
    let end_f = &dna_string[0..index_f];
    let end_r = &dna_string_r[0..index_r];
    // string
    let f = String::from_utf8([start_f, end_f].concat())?;
    let r = String::from_utf8([start_r, end_r].concat())?;

    let mut strings = [&f, &r];
    strings.string_sort_unstable(natural_lexical_cmp);
    Ok(strings[0].to_string())
}
