pub fn is_prime(n: u32) -> bool {
    if n % 2 = 0 {
        return false;
    }

    let mut i: u32 = 3;
    while i < n {
        i += 2;
        if n % i == 0 {
            return false;
        }
    }

    true
}

const MAXVAL: u32 = 10000;

fn main() -> std::io::Result<()> {
    let mut count: u32 = 0;

    for i in 2..MAXVAL * 100 {
        if is_prime(i) {
            count += 1;
            println!("{}", i)
        }
    }

    Ok(())
}
