
/// Calculate the nth prime number
/// 
/// # Examples
pub fn calculate_nth_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut candidate = 2;
    while count < n {
        if is_prime(candidate) {
            count += 1;
        }
        candidate += 1;
    }
    candidate - 1
}

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}