# My (Max's?) Minimal Fasta Toolkit

Nothing sophisticated. Minimal, simple fasta tools. 

It's stuff that's been done many times, but this way I can get it just the way I like it. May be of use to others.

## Usage

### Calculations

- `mmft len <fasta(s)>` or `cat <fasta> | mmft len`. Calculates lengths of each fasta record.
- `mmft gc <fasta(s)>` or `cat <fasta> | mmft gc`. Calculates GC content of each fasta record.
- `mmft n50 <fasta(s)>` or `cat <fasta> | mmft n50`. Calculates n50 of a fasta record (or stream of fasta files combined).

### File manipulations

- `mmft regex -r "<regex>" <fasta(s)>` or `cat <fasta> | mmft regex -r "<regex>"`. Extracts fasta records from one or multiple fasta files with headers matching the regex. 

Careful when piping, as multiple fasta files (and the records within) will be merged in the n50 calculations.

All printed to STDOUT.

## TODO's

I'll add stuff as and when I have time, or they are of use. Maybe:

- Simple pattern matching, returning positions.
- Potential ORFs
- Any kmer stuff? 
- Extract subsequences