// src/mult_table.rs
pub fn multiplication_table_mod_n(n: u32) {
    println!("Multiplication Table (mod {})", n);
    for i in 1..=n {
        for j in 1..=n {
            print!("{:>4}", (i * j) % n);
        }
        println!();
    }
}
pub fn find_multiplicative_inverses(n: u32) -> Vec<(u32, u32)> {
    let mut inverses = vec![];

    for i in 1..n {
        for j in 1..n {
            if (i * j) % n == 1 {
                inverses.push((i, j));
                break;
            }
        }
    }

    inverses
}
