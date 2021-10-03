fn main() {
    /* VARIABLES */
    let V0: u8 = 0;
    let V1 = 0u8;
    let V2: bool = false;
    let V3: u8 = 0u8;

    /* DON'T CHANGE ANYTHING BELOW */
    println!("Rust eccentrics:");
    println!("================");

    /* for loop */
    for i in 0..V0 {
        println!("Happy ");
    }

    /* switch statement -> pattern matching */
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
    let s: &str = if V3 == 3 { "Go" } else { "Boo" };

    /* if statement */
    if V2 {
        println!("{} SOLANA!", s);
    } else {
        println!("{} ETHEREUM!", s);
    }
}
