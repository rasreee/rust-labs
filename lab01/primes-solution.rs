pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

const MAXVAL: u32 = 10000;
use std::collections::HashSet;
use std::{fs::File, io::Write};

const OUTPUT_FILE_PATH: &str = "primenumbers.txt";

fn main() -> std::io::Result<()> {
    let mut file = File::create(OUTPUT_FILE_PATH)?;
    let mut visited: HashSet<u32> = HashSet::new();
    let mut count: u32 = 0;

    let one_percent = MAXVAL as f64 * 0.01;

    for i in 2..MAXVAL * 100 {
        if is_prime(i) {
            count += 1;
            let line = format!("{}\n", i);
            file.write_all(&line.as_bytes())?;
        }

        let decimal: f64 = (i as f64 / one_percent).floor();
        let percent: u32 = decimal as u32;

        if visited.contains(&percent) {
            continue;
        }

        visited.insert(percent);
        let line = format!("{}% complete\n", percent);
        file.write_all(&line.as_bytes())?;
    }
    file.write_all(b"100% complete\n")?;

    println!("Total primes found: {}\n", count);
    let line = "Total primes found: {}\n";
    file.write_all(&line.as_bytes())?;
    Ok(())
}
