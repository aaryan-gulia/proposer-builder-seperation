use rand::Rng;
use std::io::{Result, Write};

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let filename = "random_numbers.csv";
    let mut file = std::fs::File::create(filename)?;

    // Write header
    //file.write_all(b"number\n")?;

    for _ in 0..100_000 {
        let number = rng.gen::<f64>();
        write!(file, "{}\n", number)?;
    }

    println!("Generated 100,000 random numbers and saved to {}", filename);
    Ok(())
}
