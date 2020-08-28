"""
Generation of huge prime numbers, by using the mersenne primes.

USAGE:

PYTHON mersenne.py UPPER_LIMIT

PYTHON == python3, pypy
"""

import sys

"""Prime numbers below 1000."""
primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31,
    37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
    157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211,
    223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
    277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
    349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
    419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467,
    479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557,
    563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617,
    619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683,
    691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
    769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839,
    853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
    929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997]

def primes_mil():
    """A generator of all primes below 1,000,000."""

    def is_prime_mil(n):
        for p in primes:
            if p * p > n:
                return True
            if n % p == 0:
                return False

    yield 2
    for i in range(3,1_000_000+1,2):
        if is_prime_mil(i):
            yield i

"""
Generation of big mersenne prime numbers.
"""

# Given
# m := 2 ^ p - 1    p - prime
# Theorem
# m is prime iff Seq[p-1] is divisible by p

def seq(n, modulus):
    m = 4
    while n > 1:
        m = (m * m - 2) % modulus
        n -= 1
    return m

USAGE = """\
Script usage:

$ PYTHON mersenne.py UPPER_LIMIT

  where PYTHON == python3, pypy
"""

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(USAGE)
        sys.exit(1)

    upper = int(sys.argv[1])

    for p in primes_mil():
        if p > upper:
            break

        m = 2 ** p - 1
        s = seq(p-1, m)
        isPrime = s % m == 0
        if isPrime:
            print(f"{p:3}\t2^{p}-1")
