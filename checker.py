from datetime import datetime

mersenne_primes = [2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127,
  521, 607, 1279, 2203, 2281, 3217, 4253, 4423, 9689, 9941, 11213,
  19937, 21701, 23209, 44497, 86243, 110503, 132049, 216091, 756839,
  859433, 1257787, 1398269, 2976221, 3021377, 6972593, 13466917,
  20996011, 24036583, 25964951, 30402457]


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

if __name__ == "__main__":
    for i , p in enumerate(mersenne_primes):
        start = datetime.now()
        m = 2 ** p - 1
        s = seq(p-1, m)
        isPrime = s % m == 0
        end = datetime.now()
        if isPrime:
            print(f"#{i:2}: {p:6} OK {end-start}")
