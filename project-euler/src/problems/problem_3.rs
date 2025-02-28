pub fn problem_3(mut term: i64) -> i64 {
    let mut prev_primes = vec![1];
    let mut current = 3;
    let mut max_prime = 3;
    let root = (term as f64).sqrt();
    let root_as_int = root.round() as i64;

    while term % 2 == 0 {
        // divide by 2 until it is an odd number
        term = term / 2;
    }

    while current <= root_as_int + 1 {
        if term % current == 0 {
            // If current num is a factor, divide term by it until no longer a factor
            while term % current == 0 {
                term = term / current;
            }
            max_prime = current;
            prev_primes.push(current);
        }
        // skip even numbers as our term is an odd number
        current += 2
    }
    return max_prime;
}
