Some benchmarks regarding prime numbers.

## 1. Merssenne primes

**How many Mersenne primes are there?**

| Limit  | # MP |
|  100   |   3  |
| 1000   |  13  |

**How long it takes to compute them?**

| Lang    |  Upper bound |  Time |
| Rust    |         2000 |  2.5s |
| Python3 |         2000 |  1.6s |
| pypy3   |         2000 |  1.0s |
| pypy3   |         5000 | 22.5s |
| Rust    |         5000 | 46.1s | num_bigint
| Rust    |         5000 | 11.1s | num_bigint + rayon
| Rust    |         5000 |  8.2s | rug
| Rust    |         5000 |  2.2s | rug + rayon
| Rust    |        10000 | 2m24s | num_bigint + rayon
| Rust    |        10000 | 1m33s | rug
| Rust    |        10000 |   23s | rug + rayon

Approximation:
  Time(X+rayon) = Time(X) * 0.25

## 2. Normal primes

Naive computation in rust:

| Limit | #primes | Time |
| 10^6  |   78498 | 0.3s |
| 10^7  |  664579 | 3.0s |
| 10^8  | 5761455 | 80.s |