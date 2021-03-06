use rug::Integer;
use std::env;
use std::time::Instant;

use std::sync::mpsc::{channel, Sender};
use std::thread;

const SMALL_PRIMES: [u32; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

fn generate_primes(limit: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = SMALL_PRIMES.to_vec();

    let mut current = 1001;
    while current <= limit {
        let mut is_prime = true;
        for prime in SMALL_PRIMES.iter() {
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

fn is_mersenne_prime(prime: u32) -> bool {
    let mut m = Integer::from(1) << prime;
    m -= 1;

    let s = prime_seq(prime - 1, &m);
    let is_prime = s == 0;

    is_prime
}

const N_THREADS: usize = 6;

fn generate_threads(send: Sender<u32>, primes: Vec<u32>) {

    for i in 0..N_THREADS {
        let sender = send.clone();
        let primes = primes.clone();

        thread::spawn(move || {
            let mut k = i;
            while k < primes.len() {
                let prime = primes[k];
                if is_mersenne_prime(prime) {
                    sender.send(prime).unwrap();
                }
                k += N_THREADS;
            }
        });
    }
}


fn initialize_primes() -> Vec<u32> {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1).expect("Argument required!");
    let upper_bound: u32 = arg.parse().unwrap_or(1000);

    generate_primes(upper_bound)
}

fn main() {
    let primes = initialize_primes();
    let (send, recv) = channel();

    let start = Instant::now();
    let mut count = 1;
    send.send(2).unwrap();

    generate_threads(send, primes);

    let mut values = vec![];

    // Real time printing

    for value in recv {
        println!(
            "#{:2} Got value: {:6} after {:.2?}",
            count,
            value,
            start.elapsed()
        );
        count += 1;
        values.push(value);
    }

    // Final summary

    values.sort();
    println!("===================\n\tDONE\n===================");
    for (i, val) in values.iter().enumerate() {
        println!(
            "#{:2} \t{:6}",
            i+1,
            val,
        );
    }
}
