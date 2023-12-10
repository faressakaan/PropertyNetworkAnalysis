use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, WriterBuilder};

pub fn sample_dataset(file_path: &str, sample_size: usize) -> Result<(), Box<dyn Error>> {
    // Open the .csv file
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Collect all records into a vector
    let mut records: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect());
    }

    // Shuffle the records and take the first 'sample_size' entries
    let mut rng = thread_rng();
    records.as_mut_slice().shuffle(&mut rng);
    let sampled_records = records.into_iter().take(sample_size).collect::<Vec<_>>();

    // Save the sampled records to a new .csv file
    let mut wtr = WriterBuilder::new().from_path("sampled_data.csv")?;
    for record in sampled_records {
        wtr.write_record(&record)?;
    }
    wtr.flush()?;

    Ok(())
}
