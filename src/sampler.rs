use csv::{ReaderBuilder, WriterBuilder, StringRecord};
use rand::{seq::SliceRandom, thread_rng};
use std::{error::Error, fs::File};

pub fn sample_dataset(file_path: &str, output_file_path: &str, sample_size: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let headers = rdr.headers()?.clone();

    let mut records: Vec<StringRecord> = rdr.records().filter_map(Result::ok).collect();
    let mut rng = thread_rng();
    records.as_mut_slice().shuffle(&mut rng);

    let sampled_records = records.into_iter().take(sample_size).collect::<Vec<_>>();
    let mut wtr = WriterBuilder::new().from_path(output_file_path)?;
    wtr.write_record(&headers)?;

    for record in sampled_records {
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}
