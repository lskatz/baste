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

## Etymology

This is like the fasten project but for bam files, and so I chose a similar word that starts with a B.
