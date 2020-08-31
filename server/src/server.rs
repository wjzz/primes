use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

const OK_FOUND: &'static str = "HTTP/1.1 200 Ok\r\n";
const NOT_FOUND: &'static str = "HTTP/1.1 404 Not Found\r\n\r\n";

#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct ServerPayload {
    pub prime_count: u32,
    pub biggest: u32,
    pub start: std::time::Instant,
    pub N_THREADS: usize,
    pub lower_bound: u32,
    pub upper_bound: u32,
}

fn generate_json() -> String {
    let content_type = "Content-Type: application/json\r\n";

    let json = String::from("[]");

    format!("{}\r\n{}", content_type, json)
}

fn generate_stats_html(
    primes: Arc<Mutex<Vec<u32>>>,
    checked_count: Arc<Mutex<Vec<u32>>>,
    payload: ServerPayload,
) -> String {
    let nums = primes.lock().unwrap();
    let num = nums.len();
    let ServerPayload {
        biggest,
        prime_count,
        start,
        N_THREADS,
        lower_bound,
        upper_bound,
    } = payload;

    let mut out = format!(
        "{}\r\nActive threads: {}\nBiggest prime to check: {}\n",
        OK_FOUND, N_THREADS, biggest
    );

    out.push_str(&format!("Lower bound: {}\nUpper bound: {}\n", lower_bound, upper_bound));

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
        out.push_str(&format!(" Thread {} did: {}\n", i + 1, n));
    }

    out.push_str(&format!("\n"));

    let elapsed = start.elapsed();
    out.push_str(&format!("Time elapsed: {:.2?}\n", elapsed));
    let elapsed_per_prime = elapsed.as_millis() as f64 / sum as f64;
    out.push_str(&format!(
        "Time elapsed per prime: {:.2?}ms\n",
        elapsed_per_prime
    ));
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

pub fn handle_client(
    mut stream: TcpStream,
    primes: Arc<Mutex<Vec<u32>>>,
    checked_count: Arc<Mutex<Vec<u32>>>,
    payload: ServerPayload,
) {
    // println!("==================");
    // println!("Connection incoming!");

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    // print!("Request: {}", request);

    let header = request.lines().nth(0).unwrap();
    // println!("First line: {}", header);

    let resource = if String::from(header).starts_with("GET") {
        &header.split(' ').nth(1).unwrap()[1..]
    } else {
        ""
    };
    // println!("PATH: {}", resource);

    let response = match resource {
        "" => format!("{}\r\n{}", OK_FOUND, serve_www_file("index.html")),
        "main.js" => format!("{}\r\n{}", OK_FOUND, serve_www_file("main.js")),
        "count" => generate_stats_html(primes, checked_count, payload),
        "json" => format!("{}{}", OK_FOUND, generate_json()),
        _ => NOT_FOUND.to_owned(),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Closing connection!");
}

//============================================================================
// SERVER INITIALIZATION

const PORT: u32 = 8080;

pub fn server_main(
    found_mersennes: &Arc<Mutex<Vec<u32>>>,
    checked_count: &Arc<Mutex<Vec<u32>>>,
    payload: ServerPayload,
) -> std::io::Result<()> {
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
