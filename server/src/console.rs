use std::sync::mpsc::Receiver;
use std::time::Instant;

pub fn console_reporter(recv: Receiver<u32>) {
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
