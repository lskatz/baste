extern crate noodles;
use noodles::bam as bam;

fn main() {

    let path = "/dev/stdin";

    let mut reader = bam::io::reader::Builder::default().build_from_path(&path).expect("error opening BAM file");
    let _header = reader.read_header().expect("error reading BAM header");

    let mut num_records = 0;
    let mut seq_length = vec![];
    let mut qual_scores: Vec<u8> = vec![];
    for result in reader.records() {
        num_records += 1;

        let record = result.expect(&format!("error reading BAM record number {}", num_records));

        let seq  = record.sequence();
        let qual = record.quality_scores();
        let id   = record.name()
                        .expect(&format!("ERROR: read name missing for record number {}", num_records));
        let length = seq.len();
        if qual.len() != length {
            eprintln!("Warning: sequence and quality score lengths do not match for record {}", &id);
        }

        seq_length.push(seq.len());
        qual_scores.extend(qual.as_ref());
    }

    println!("totalReadLength\tnumReads\tavgReadLength\tavgQual");
    let total_read_length: usize = seq_length.iter().sum();
    let avg_read_length = total_read_length as f64 / num_records as f64;
    let avg_qual_score = avg_qual(&qual_scores, 0);
    println!("{}\t{}\t{:.2}\t{:.2}", total_read_length, num_records, avg_read_length, avg_qual_score);
}

/// Calculates average quality value from a vector of quality bytes
fn avg_qual(qual: &[u8], ascii_base: u8) -> f64 {
    if qual.is_empty() {
        return 0.0;
    }

    // Parse quality values (assuming Phred scores)
    let qual_values: Vec<f64> = qual.iter()
        .map(|&q| {
            let phred = (q - ascii_base) as i16;
            // Convert Phred to probability: P = 10^(-Q/10)
            10_f64.powf(-phred as f64 / 10.0)
        })
        .collect();

    if qual_values.is_empty() {
        return 0.0;
    }
                                                                                                                                                
    let sum: f64 = qual_values.iter().sum();                                                                                                    
    let avg_prob = sum / qual_values.len() as f64;                                                                                              
                                                                                                                                                
    // Convert average probability back to Phred score                                                                                          
    -10.0 * avg_prob.log10()                                                                                                                    
}
