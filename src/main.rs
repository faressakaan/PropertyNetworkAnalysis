mod sampler;

fn main() {
    let file_path = "/Users/faressakaan/Downloads/Real_Estate_Sales_2001-2020_GL.csv"; 
    let sample_size = 10_000; // Your desired sample size

    match sampler::sample_dataset(file_path, sample_size) {
        Ok(_) => println!("Sampling completed successfully."),
        Err(e) => eprintln!("Error occurred during sampling: {}", e),
    }
}
