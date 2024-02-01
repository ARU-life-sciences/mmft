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

### File manipulations

- `mmft regex -r "<regex>" <fasta(s)>` or `cat <fasta> | mmft regex -r "<regex>"`. Extracts fasta records from one or multiple fasta files with headers matching the regex. 
- `mmft extract -r 1-100 <fasta(s)>` or `cat <fasta> | mmft extract -r 1-100`. Extracts first 100 nucleotides from each fasta record. You can of course choose any range, using a dash to separate the numbers.
- `mmft filter -f <file> <fasta(s)>`. Supply a text file of one ID per line and filter will extract the corresponding fasta records.
- `mmft merge <fastas>`. Will merge multiple fasta files together into the same record.
- `mmft sample <fasta(s)> -n <N>`. Will randomly sample a fasta file (or stream of fasta files) to a specified number of records.

Careful when piping into `mmft` as fasta files are not treated separately, they are treated as a continuum of fasta records. Hence, while `mmft n50 1.fasta 2.fasta` shows the n50 of each fasta file separately, `cat *.fasta | mmft n50` will calculate the n50 of both files combined. In addition, `mmft sample` loads the entire STDIN into memory, so be careful when piping large files.

All printed to STDOUT.

## TODO's

I'll add stuff as and when I have time, or they are of use. Maybe:

- Simple pattern matching, returning positions.
- Potential ORFs
- Any kmer stuff?
- Testing
- Better documentation
