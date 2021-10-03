fn main() {
    println!("Rust eccentrics:");
    println!("================");

    /* for loop */
    let V0: u8 = 0;
    for i in 0..V0 {
        println!("Happy ");
    }

    /* switch statement -> pattern matching */
    let V1 = 0u8;
    match V1 {
        1 => println!("Triangle Man"),
        2 => println!("Chinese Erhu Guy"),
        3 => println!("Huh"),
        4 => println!("Yoshua"),
        5 => println!("Dr. Jokemon"),
        6 => println!("Hat Lady"),
        _ => println!("I don't know these people..."),
    }

    /* ternary operator */
    let V2: bool = false;
    let V3: u8 = 0u8;
    let s: &str = if V3 == 3 { "Go" } else { "Boo" };

    /* if statement */
    if V2 {
        println!("{} SOLANA!", s);
    } else {
        println!("{} ETHEREUM!", s);
    }
}
