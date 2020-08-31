use rug::Integer;

fn prime_seq(mut n: u32, modulus: &Integer) -> Integer {
    let mut m = Integer::from(4);

    while n > 1 {
        m.square_mut();
        m -= 2;
        m %= modulus;
        n -= 1;
    }
    m
}

pub fn is_mersenne_prime(prime: u32) -> bool {
    let mut m = Integer::from(1) << prime;
    m -= 1;

    let s = prime_seq(prime - 1, &m);
    let is_prime = s == 0;

    is_prime
}
