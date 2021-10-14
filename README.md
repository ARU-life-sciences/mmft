# My (Max's?) Minimal Fasta Toolkit

Nothing sophisticated. Minimal, simple fasta tools. 

It's stuff that's been done many times, but this way I can get it just the way I like it. May be of use to others.

## Usage

### Calculations

- `mmft len <fasta(s)>` or `cat <fasta(s)> | mmft len`. Calculates lengths of each fasta record.
- `mmft gc <fasta(s)>` or `cat <fasta(s)> | mmft gc`. Calculates GC content of each fasta record.
- `mmft n50 <fasta(s)>` or `cat <fasta(s)> | mmft n50`. Calculates n50 of a fasta record (or stream of fasta files combined).
- `mmft num <fasta(s)>` or `cat <fasta(s)> | mmft num`. Calculates number of sequences, and total number of base pairs in the fasta file input(s).

### File manipulations

- `mmft regex -r "<regex>" <fasta(s)>` or `cat <fasta> | mmft regex -r "<regex>"`. Extracts fasta records from one or multiple fasta files with headers matching the regex. 
- `mmft extract -r 1-100 <fasta(s)>` or `cat <fasta> | mmft extract -r 1-100`. Extracts first 100 nucleotides from each fasta record. You can of course choose any range, using a dash to separate the numbers.

Careful when piping into `mmft` as fasta files are not treated separately, they are treated as a continuum of fasta records. Hence, while `mmft n50 1.fasta 2.fasta` shows the n50 of each fasta file separately, `cat *.fasta | mmft n50` will calculate the n50 of both files combined. 

All printed to STDOUT.

## TODO's

I'll add stuff as and when I have time, or they are of use. Maybe:

- Simple pattern matching, returning positions.
- Potential ORFs
- Any kmer stuff?

### Motivation

Interesting to explore the `thiserror` and the `anyhow` crates in the making of this repo.