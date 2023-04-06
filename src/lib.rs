pub fn gcd(n: u32, m: u32) -> u32 {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}
