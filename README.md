# My (Max's?) Minimal Fasta Toolkit

Minimal, simple fasta tools.

Each program is self-contained in the `./src/fasta` directory, and follows similar boilerplate code, related to file handling. So if you feel like contributing and/or adding your own subcommand, please do.

## Usage

Typing `mmft` (shows subcommands) or `mmft <subcommand> -h` (shows specific subcommand) will show the usage of the tool in question.

Commands are added only as and when I need them. If you like what you see, please feel free to contribute a PR with your favourite subcommand.

### Calculations

- `mmft len <fasta(s)>` or `cat <fasta(s)> | mmft len`. Calculates lengths of each fasta record.
- `mmft gc <fasta(s)>` or `cat <fasta(s)> | mmft gc`. Calculates GC content of each fasta record.
- `mmft n50 <fasta(s)>` or `cat <fasta(s)> | mmft n50`. Calculates n50 of a fasta record (or stream of fasta files combined).
- `mmft num <fasta(s)>` or `cat <fasta(s)> | mmft num`. Calculates number of sequences, and total number of base pairs in the fasta file input(s).
- `mmft revcomp <fasta(s)>` or `cat <fasta(s) | mmft revcomp`. Reverse complements each record in the fasta file.
- `mmft min <fasta(s)>` or `cat <fasta(s) | mmft min`. Minimally lexicographically rotated string returned. Takes into account reverse complement too.
- `mmft stats <fasta(s)>` or `cat <fasta(s)> | mmft stats`. Per-file assembly statistics in one pass: sequence count, total bp, GC%, N%, min/max length, N50/L50, and auN (area under the Nx curve).
- `mmft telomere <fasta(s)> (-m <MOTIF>) (-w <WINDOW>)`. Scans the ends of each record for canonical telomere repeat motifs (or a motif you supply), reporting the repeat count found at each end.
- `mmft skew <fasta(s)> (-w <WINDOW>)`. Calculates GC skew, `(G-C)/(G+C)`, per window along each sequence, along with its cumulative sum (useful for spotting origins/termini of replication in circular genomes).

### File manipulations

- `mmft regex -r "<regex>" <fasta(s)>` or `cat <fasta> | mmft regex -r "<regex>"`. Extracts fasta records from one or multiple fasta files with headers matching the regex. 
- `mmft extract -r 1-100 <fasta(s)>` or `cat <fasta> | mmft extract -r 1-100`. Extracts first 100 nucleotides from each fasta record. You can of course choose any range, using a dash to separate the numbers.
- `mmft filter -f <file> <fasta(s)>`. Supply a text file of one ID per line and filter will extract the corresponding fasta records.
- `mmft merge <fastas>`. Will merge multiple fasta files together into the same record.
- `mmft sample <fasta(s)> -n <N>`. Will randomly sample a fasta file (or stream of fasta files) to a specified number of records.
- `mmft split (-d <DIR>) -n <N> <fasta(s)>`. Splits fasta into equal chunks with the last chunk the remainder if record number not perfectly divisible by chunk number. 

Careful when piping into `mmft` as fasta files are not treated separately, they are treated as a continuum of fasta records. Hence, while `mmft n50 1.fasta 2.fasta` shows the n50 of each fasta file separately, `cat *.fasta | mmft n50` will calculate the n50 of both files combined. `mmft sample` and `mmft split` don't support piping at all (they need to be able to seek/reopen the input file), and `mmft filter` reads its ID list fully into memory before filtering.

All printed to STDOUT.

## TODO's

I'll add stuff as and when I have time, or they are of use. Maybe:

- Single/multiline fasta exchange.
- Simple pattern matching, returning positions.
- Potential ORFs
- Any kmer stuff?
- Testing
- Better documentation
