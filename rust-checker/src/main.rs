use std::time::Instant;

mod prime_generator;
mod primes;
mod time;

const MERSENNE_PRIMES: [u32; 43] = [2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127,
  521, 607, 1279, 2203, 2281, 3217, 4253, 4423, 9689, 9941, 11213,
  19937, 21701, 23209, 44497, 86243, 110503, 132049, 216091, 756839,
  859433, 1257787, 1398269, 2976221, 3021377, 6972593, 13466917,
  20996011, 24036583, 25964951, 30402457];

fn main() {
    // Generate the primes from the given span
    println!("Verying Mersenne primes...");

    for (i, &prime) in MERSENNE_PRIMES.iter().enumerate() {
        print!("Checking #{:2} = {:9}  ", i+1, prime);
        let now = Instant::now();

        let is_prime = primes::is_mersenne_prime(prime);

        let msg = if is_prime { "OK" } else { "NG" };
        let elapsed = now.elapsed().as_millis();
        println!("{} [{}]", msg, time::format_time(elapsed));
    }
}
