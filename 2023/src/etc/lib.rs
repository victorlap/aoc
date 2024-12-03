pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }

    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}
