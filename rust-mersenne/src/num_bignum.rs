use std::env;
use num_bigint::{BigUint, ToBigUint};
use rayon::prelude::*;

fn literal(n: u32) -> BigUint {
    n.to_biguint().unwrap()
}

fn prime_seq(mut n: u32, modulus: &BigUint) -> BigUint {
    let mut m = literal(4);

    while n > 1 {
        m = m.pow(2);
        m -= 2u32;
        m %= modulus;
        n -= 1;
    }
    m
}

fn generate_primes(limit: u32) -> Vec<u32> {
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];

    let mut vec: Vec<u32> = primes.to_vec();

    let mut current = 1001;
    while current <= limit {
        let mut is_prime = true;
        for prime in primes.iter() {
            if current % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            vec.push(current);
        }
        current += 2;
    }

    return vec
      .iter()
      .by_ref()
      .take_while(|&e| e <= &limit)
      .cloned()
      .collect();
}

fn check_prime(prime: u32, zero: &BigUint, one: &BigUint, two: &BigUint) -> () {
    let m = two.pow(prime) - one;
    let s = prime_seq(prime - 1, &m);
    let is_prime = s % m == *zero;

    if is_prime {
        println!("{}", prime);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1).expect("Argument required!");
    let upper_bound: u32 = arg.parse().unwrap_or(1000);

    let primes = generate_primes(upper_bound);

    let zero = literal(0);
    let one = literal(1);
    let two = literal(2);

    primes
        .par_iter()
        .for_each(|&prime| check_prime(prime, &zero, &one, &two));
}
