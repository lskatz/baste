# Baste

This is an initial attempt at a bam file manipulation tool in Rust.

## Usage

```bash
cat something.bam | baste_metrics | column -t

totalReadLength  numReads  avgReadLength  avgQual
2000000          20000     100.00         30.44
```

```bash
cat something.bam | baste_convert | gzip -c > out.fastq.gz
```

## Installation

```bash
cargo build --release
# => executables can be found in ./target/release
```

## Contributing

Please send a message on issues if you want to contribute.
This is a proof of concept so far and so there is lots of room for helping.

## Etymology

This is like the fasten project but for bam files, and so I chose a similar word that starts with a B.
