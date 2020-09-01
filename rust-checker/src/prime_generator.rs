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

#[allow(dead_code)]
pub fn generate_primes(upper_limit: u32) -> Vec<u32> {
    generate_primes_gen(2, upper_limit)
}

pub fn generate_primes_gen(lower_limit: u32, upper_limit: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![];

    let mut current = lower_limit;
    if current > 2 && current % 2 == 0 {
        current += 1;
    }

    while current <= upper_limit {
        let mut is_prime = true;
        if current <= 1_000_000 {
            for &prime in SMALL_PRIMES.iter() {
                if current % prime == 0 && prime < current {
                    is_prime = false;
                    break;
                }
            }
        } else {
            let mut divisor = 3;
            while divisor * divisor <= current {
                if current % divisor == 0 {
                    is_prime = false;
                    break;
                }
                divisor += 2;
            }
        }

        if is_prime {
            primes.push(current);
        }

        if current == 2 {
            current += 1;
        } else {
            current += 2;
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    mod generate_primes_tests {
        use super::super::*;

        #[test]
        fn upto_0() {
            let primes = generate_primes(0);
            assert_eq!(primes.len(), 0);
        }

        #[test]
        fn upto_10() {
            let primes = generate_primes(10);
            assert_eq!(primes.len(), 4);
        }

        #[test]
        fn upto_100() {
            let primes = generate_primes(100);
            assert_eq!(primes.len(), 25);
        }

        #[test]
        fn upto_1_000_000() {
            let primes = generate_primes(1_000_000);
            assert_eq!(primes.len(), 78498);
        }

        #[test]
        fn upto_10_000_000() {
            let primes = generate_primes(10_000_000);
            assert_eq!(primes.len(), 664579);
        }
    }

    mod generate_primes_gen_tests {
        use super::super::*;

        #[test]
        fn from_2_to_10() {
            let primes = generate_primes_gen(2, 10);
            assert_eq!(primes.len(), 4);
        }

        #[test]
        fn from_10_to_2() {
            let primes = generate_primes_gen(10, 2);
            assert_eq!(primes.len(), 0);
        }

        #[test]
        fn from_90_to_100() {
            let primes = generate_primes_gen(90, 100);
            assert_eq!(primes.len(), 1);
        }

        #[test]
        fn from_100k_to_110k() {
            let primes = generate_primes_gen(100_000, 110_000);
            assert_eq!(primes.len(), 861);
        }
    }
}
