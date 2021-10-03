use std::{fs::File, io::Write};

const OUTPUT_FILE_PATH: &str = "primenumbers.txt";

fn main() -> std::io::Result<()> {
    let mut file = File::create(OUTPUT_FILE_PATH)?;
    file.write_all(b"Output redirection testing....")?;

    for i in 0..10 {
        let line = format!("\nWriting again for the {} time...", i);
        file.write_all(&line.as_bytes())?;
    }
    Ok(())
}
