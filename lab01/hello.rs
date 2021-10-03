fn main() {
    let mut count: u8 = 0;
    let p: &mut u8 = &mut count;

    for i in 0..10 {
        *p += 1;
    }

    println!("Have a nice day.");
}
