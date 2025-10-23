extern crate noodles;
use noodles::bam as bam;
use getopts::Options;

fn main() {

    let path = "/dev/stdin";

    let mut reader = bam::io::reader::Builder::default().build_from_path(&path).expect("error opening BAM file");
    let _header = reader.read_header().expect("error reading BAM header");

    let matches = Options::new()
        .optopt("o", "out-format", "output format (default: fastq)", "FORMAT")
        .parse(&std::env::args().collect::<Vec<String>>())
        .expect("error parsing command line options");

    let mut num_records = 0;
    for result in reader.records() {
        num_records += 1;

        let record = result.expect(&format!("error reading BAM record number {}", num_records));
println!("record {:?}\n{:?}\n{:?}", &record, &record.data(), &record.flags());
        let seq  = record.sequence();
        let qual = record.quality_scores();
        let id   = record.name()
                        .expect(&format!("ERROR: read name missing for record number {}", num_records));
        let mate = record.mate_reference_sequence_id()
                        .expect(&format!("ERROR: mate reference sequence ID missing for read {}", &id))
                        .unwrap();
        let sequence: String = seq.iter().map(|base| char::from(base)).collect();
        println!("@{} mate:{}\n{}\n+\n{}", id, mate, sequence, &quality_to_ascii(&qual));
    }

}

fn quality_to_ascii(qual: &bam::record::QualityScores) -> String {
    qual.iter().map(|q| (q + 33) as char).collect()
}
