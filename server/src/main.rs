use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// Local files

mod args;
mod console;
mod prime_generator;
mod primes;
mod server;
use server::ServerPayload;

// Constants

const N_THREADS: usize = 1;

// A main worker thread

fn spawn_threads(
    send: Sender<u32>,
    found_mersennes: &Arc<Mutex<Vec<u32>>>,
    checked_count: &Arc<Mutex<Vec<u32>>>,
    primes: Vec<u32>,
) {
    for i in 0..N_THREADS {
        let sender = send.clone();
        let primes = primes.clone();
        let found_mersennes_clone = Arc::clone(found_mersennes);
        let checked_count_clone = Arc::clone(checked_count);

        thread::spawn(move || {
            let mut k = i;

            while k < primes.len() {
                let prime = primes[k];
                if primes::is_mersenne_prime(prime) {
                    sender.send(prime).unwrap();
                    let mut vec = found_mersennes_clone.lock().unwrap();
                    vec.push(prime);
                }
                let mut counts = checked_count_clone.lock().unwrap();
                counts[i] += 1;
                k += N_THREADS;
            }
        });
    }
}

// Putting it all together

fn main() {
    // Generate the primes from the given span
    println!("Generating primes...");
    let (lower_bound, upper_bound) = args::parse_cmd_args();
    println!("Looking for Mersenne primes in [{},{}]", lower_bound, upper_bound);

    let primes = prime_generator::generate_primes_gen(lower_bound, upper_bound);

    let prime_count = primes.len() as u32;
    let biggest = primes[primes.len() - 1];
    println!("Generated {} primes...", prime_count);

    // Initialize synchronization channels and mutexes
    let (send, recv) = channel();
    let found_mersennes = Arc::new(Mutex::new(vec![]));
    let checked_count = Arc::new(Mutex::new(vec![0; N_THREADS]));

    // 2 is a mersenne prime, but it fails the tests
    if lower_bound <= 2 && 2 <= upper_bound {
        let mut m = found_mersennes.lock().unwrap();
        m.push(2);
        send.send(2).unwrap();
    }

    // Spawn all the threads
    println!("Spawning {} worker threads...", N_THREADS);
    spawn_threads(send, &found_mersennes, &checked_count, primes);

    println!("Spawning the console reporter");
    let reporter = thread::spawn(move || {
        console::console_reporter(recv);
    });

    let start = Instant::now();
    let server_payload = ServerPayload {
        prime_count,
        biggest,
        start,
        N_THREADS,
        lower_bound,
        upper_bound,
    };

    println!("Spawning the server reporter");
    let server = std::thread::spawn(move || {
        server::server_main(&found_mersennes, &checked_count, server_payload).unwrap();
    });

    // Wait for the reports to finish (the server never really finishes though)
    reporter.join().unwrap();
    server.join().unwrap();
}
