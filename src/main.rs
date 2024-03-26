use num_cpus;
use std::thread;
use stresser_rs::calculate_nth_prime;

fn main() {
    let num_cores = num_cpus::get();
    let mut threads = Vec::with_capacity(num_cores);

    for _ in 0..num_cores {
        threads.push(thread::spawn(|| {
            // Call the CPU-intensive function here
            let nth_prime = calculate_nth_prime(1_000_000);
            println!("The 1,000,000th prime number is: {}", nth_prime);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}