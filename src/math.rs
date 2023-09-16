/// gcd: greatest common divisor
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// lcm: least common multiple
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
