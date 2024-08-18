// Function to compute the greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to compute the least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// Function to compute the additive order of a in Z_n
fn additive_order(a: u64, n: u64) -> u64 {
    n / gcd(a, n)
}

// Function to compute the order of (a, b) in Z_n x Z_m
fn order(a: u64, n: u64, b: u64, m: u64) -> u64 {
    let add_order_a = additive_order(a, n);
    let add_order_b = additive_order(b, m);
    lcm(add_order_a, add_order_b)
}

fn main() {
    let a = 6;
    let n = 8;
    let b = 9;
    let m = 12;

    let result = order(a, n, b, m);
    println!("The order of ({}, {}) in Z_{} x Z_{} is {}", a, b, n, m, result);
}
