use std::env;
use std::time::Instant;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

const OK_FOUND: &'static str = "HTTP/1.1 200 Ok\r\n\r\n";
const NOT_FOUND: &'static str = "HTTP/1.1 404 Not Found\r\n\r\n";

mod primes;

const N_THREADS: usize = 6;

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

fn initialize_primes() -> Vec<u32> {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1).expect("Argument required!");
    let upper_bound: u32 = arg.parse().unwrap_or(1000);

    primes::generate_primes(upper_bound)
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

fn main() {
    let primes = initialize_primes();
    let prime_count = primes.len() as u32;
    let biggest = primes[primes.len()-1];

    let (send, recv) = channel();

    let found_mersennes = Arc::new(Mutex::new(vec![2]));
    let checked_count = Arc::new(Mutex::new(vec![0;N_THREADS]));

    send.send(2).unwrap();

    spawn_threads(send, &found_mersennes, &checked_count, primes);

    let reporter = thread::spawn(move || {
        console_reporter(recv);
    });

    let primes_list = Arc::clone(&found_mersennes);
    let checked_count_copy = Arc::clone(&checked_count);
    let server = std::thread::spawn(move || {
        server_main(primes_list, checked_count_copy, prime_count, biggest).unwrap();
    });

    reporter.join().unwrap();
    server.join().unwrap();
}

//============================================================================

fn generate_stats_html(primes: Arc<Mutex<Vec<u32>>>, checked_count: Arc<Mutex<Vec<u32>>>, prime_count: u32, biggest: u32) -> String {
    let nums = primes.lock().unwrap();
    let num = nums.len();
    // *num += 1;
    let mut out = format!(
        "{}Active threads: {}\nBiggest prime to check: {}\n",
        OK_FOUND, N_THREADS, biggest
    );

    out.push_str(&format!("\n"));

    let counts = checked_count.lock().unwrap();
    let mut sum = 0;
    for n in counts.iter() {
        sum += n;
    }
    out.push_str(&format!("Total done: {}\n", sum));

    for (i, n) in counts.iter().enumerate() {
        out.push_str(&format!(" Thread {} did: {}\n", i+1, n));
    }

    out.push_str(&format!("Total primes to check: {}\n", prime_count));

    let percentage = 100f64 * (sum as f64) / (prime_count as f64);
    out.push_str(&format!("{:.2}% done.\n", percentage));


    out.push_str(&format!("\n"));


    out.push_str(&format!("Current count: {}\n", num));

    let mut locals = nums.clone();
    locals.sort();
    for (i, p) in locals.iter().enumerate() {
        out.push_str(&format!("{:2}: {}\n", i + 1, p));
    }
    out
}

fn handle_client(mut stream: TcpStream, primes: Arc<Mutex<Vec<u32>>>, checked_count: Arc<Mutex<Vec<u32>>>, prime_count: u32, biggest: u32) {
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
        "" => format!("{}{}", OK_FOUND, "Hello from Rust!"),
        "count" => generate_stats_html(primes, checked_count, prime_count, biggest),
        _ => NOT_FOUND.to_owned(),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Closing connection!");
}

const PORT: u32 = 8080;

fn server_main(found_mersennes: Arc<Mutex<Vec<u32>>>, checked_count: Arc<Mutex<Vec<u32>>>, prime_count: u32, biggest: u32) -> std::io::Result<()> {
    println!("Server started");

    let ip = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(ip)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let primes = Arc::clone(&found_mersennes);
        let checked_count_copy = Arc::clone(&checked_count);
        thread::spawn(move || {
            handle_client(stream.unwrap(), primes, checked_count_copy, prime_count, biggest);
        });
    }
    Ok(())
}
