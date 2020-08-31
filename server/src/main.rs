use std::env;
use std::time::Instant;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use std::fs::File;
use std::path::Path;

// Local files

mod prime_generator;
mod primes;

// Constants

const OK_FOUND: &'static str = "HTTP/1.1 200 Ok\r\n";
const NOT_FOUND: &'static str = "HTTP/1.1 404 Not Found\r\n\r\n";
const N_THREADS: usize = 2;

fn spawn_threads(send: Sender<u32>, found_mersennes: &Arc<Mutex<Vec<u32>>>, checked_count: &Arc<Mutex<Vec<u32>>>, primes: Vec<u32>) {
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

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}

fn initialize_primes() -> Vec<u32> {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1).expect("Argument required!");
    let upper_bound: u32 = strip_characters(arg, "_").parse().unwrap_or(1000);

    let v = prime_generator::generate_primes(upper_bound);
    v

    // TODO: update this
    // let v = prime_generator::generate_primes(upper_bound);

    // let lower_bound = 100_000_000;
    // let mut primes = vec![];
    // for p in v {
    //     if p >= lower_bound {
    //         primes.push(p);
    //     }
    // }
    // primes
}

fn console_reporter(recv: Receiver<u32>) {
    let start = Instant::now();
    let mut count = 1;

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
        println!("#{:2} \t{:6}", i + 1, val,);
    }
}

#[derive(Clone, Copy)]
struct ServerPayload {
    prime_count: u32,
    biggest: u32,
    start: std::time::Instant,
}

fn main() {
    println!("Generating primes...");
    let primes = initialize_primes();

    // primes.reverse();
    let prime_count = primes.len() as u32;
    let biggest = primes[primes.len()-1];
    println!("Generated {} primes...", prime_count);

    let (send, recv) = channel();

    let found_mersennes = Arc::new(Mutex::new(vec![2]));
    let checked_count = Arc::new(Mutex::new(vec![0;N_THREADS]));

    send.send(2).unwrap();

    println!("Spawning {} worker threads...", N_THREADS);

    spawn_threads(send, &found_mersennes, &checked_count, primes);

    println!("Spawning the console reporter");

    let reporter = thread::spawn(move || {
        console_reporter(recv);
    });

    let start = Instant::now();
    let primes_list = Arc::clone(&found_mersennes);
    let checked_count_copy = Arc::clone(&checked_count);
    let server_payload = ServerPayload { prime_count, biggest, start };

    println!("Spawning the server reporter");
    let server = std::thread::spawn(move || {
        server_main(primes_list, checked_count_copy, server_payload).unwrap();
    });

    reporter.join().unwrap();
    server.join().unwrap();
}

//============================================================================

fn generate_json() -> String {
    let content_type = "Content-Type: application/json\r\n";

    let json = String::from("[]");

    format!("{}\r\n{}", content_type, json)
}

fn generate_stats_html(primes: Arc<Mutex<Vec<u32>>>, checked_count: Arc<Mutex<Vec<u32>>>, payload: ServerPayload) -> String {
    let nums = primes.lock().unwrap();
    let num = nums.len();
    let ServerPayload { biggest, prime_count, start} = payload;

    let mut out = format!(
        "{}\r\nActive threads: {}\nBiggest prime to check: {}\n",
        OK_FOUND, N_THREADS, biggest
    );

    out.push_str(&format!("\n"));

    out.push_str(&format!("Total primes to check: {}\n", prime_count));

    let counts = checked_count.lock().unwrap();
    let mut sum = 0u64;
    for n in counts.iter() {
        sum += *n as u64;
    }

    let percentage = 100f64 * (sum as f64) / (prime_count as f64);
    out.push_str(&format!("{:.2}% done.\n", percentage));

    out.push_str(&format!("Total done: {}\n", sum));

    for (i, n) in counts.iter().enumerate() {
        out.push_str(&format!(" Thread {} did: {}\n", i+1, n));
    }

    out.push_str(&format!("\n"));

    let elapsed = start.elapsed();
    out.push_str(&format!("Time elapsed: {:.2?}\n", elapsed));
    let elapsed_per_prime =  elapsed.as_millis() as f64 / sum as f64;
    out.push_str(&format!("Time elapsed per prime: {:.2?}ms\n", elapsed_per_prime));
    let primes_per_second = sum as f64 / elapsed.as_secs_f64();
    out.push_str(&format!("Primes per second: {:.2?}\n", primes_per_second));

    out.push_str(&format!("\n"));

    out.push_str(&format!("Current count: {}\n", num));

    let mut locals = nums.clone();
    locals.sort();
    for (i, p) in locals.iter().enumerate() {
        out.push_str(&format!("{:2}: {}\n", i + 1, p));
    }
    out
}

fn serve_www_file(path: &str) -> String {
    let base = Path::new("./src/www");
    let mut file = File::open(base.join(path)).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

fn handle_client(mut stream: TcpStream, primes: Arc<Mutex<Vec<u32>>>, checked_count: Arc<Mutex<Vec<u32>>>, payload: ServerPayload) {
    println!("==================");
    println!("Connection incoming!");

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    // print!("Request: {}", request);

    let header = request.lines().nth(0).unwrap();
    println!("First line: {}", header);

    let resource = if String::from(header).starts_with("GET") {
        &header.split(' ').nth(1).unwrap()[1..]
    } else {
        ""
    };
    println!("PATH: {}", resource);

    let response = match resource {
        "" => format!("{}\r\n{}", OK_FOUND, serve_www_file("index.html")),
        "main.js" => format!("{}\r\n{}", OK_FOUND, serve_www_file("main.js")),
        "count" => generate_stats_html(primes, checked_count, payload),
        "json" => format!("{}{}", OK_FOUND, generate_json()),
        _ => NOT_FOUND.to_owned(),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Closing connection!");
}

const PORT: u32 = 8080;

fn server_main(found_mersennes: Arc<Mutex<Vec<u32>>>, checked_count: Arc<Mutex<Vec<u32>>>, payload: ServerPayload) -> std::io::Result<()> {
    println!("Server started");

    let ip = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(ip)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let primes = Arc::clone(&found_mersennes);
        let checked_count_copy = Arc::clone(&checked_count);
        thread::spawn(move || {
            handle_client(stream.unwrap(), primes, checked_count_copy, payload);
        });
    }
    Ok(())
}
