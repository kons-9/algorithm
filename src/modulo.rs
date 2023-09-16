/// mod_pow(x, n, m) returns x^n mod m.
pub fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    if n == 0 {
        1
    } else if n & 1 == 0 {
        mod_pow((x * x) % m, n >> 1, m)
    } else {
        x * mod_pow(x, n - 1, m) % m
    }
}
