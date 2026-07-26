// Reverse complement a string slice.
pub fn reverse_complement(dna: &[u8]) -> Vec<u8> {
    static COMPLEMENT: [u8; 256] = make_complement_table();

    dna.iter()
        .rev()
        .map(|&base| COMPLEMENT[base as usize])
        .collect()
}

// Const function to generate the complement lookup table at compile time.
// Anything that isn't a recognised base maps to 'N', matching the previous
// match-based implementation's fallback.
const fn make_complement_table() -> [u8; 256] {
    let mut table = [b'N'; 256];
    table[b'A' as usize] = b'T';
    table[b'T' as usize] = b'A';
    table[b'C' as usize] = b'G';
    table[b'G' as usize] = b'C';
    table[b'a' as usize] = b't';
    table[b't' as usize] = b'a';
    table[b'c' as usize] = b'g';
    table[b'g' as usize] = b'c';
    table[b'N' as usize] = b'N';
    table[b'n' as usize] = b'n';
    table
}
