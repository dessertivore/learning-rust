use std::num::NonZeroUsize;

use lru::LruCache;

fn problem_1() {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("The sum of all multiples of 3 or 5 below 1000 is: {}", sum);
}

fn fib(term: i64, cache: &mut LruCache<i64, i64>) -> i64 {
    if term == 0 {
        return 0;
    }
    if term == 1 {
        return 1;
    }
    if term == 2 {
        return 2;
    } else {
        if let Some(&result) = cache.get(&term) {
            return result;
        }
        let result = fib(term - 1, cache) + fib(term - 2, cache);
        cache.put(term, result);
        result
    }
}

fn problem_2(term: i64, cache: &mut LruCache<i64, i64>) -> i64 {
    let mut sum = 0;
    let mut fib_num = 0;
    let mut idx = 0;
    while fib_num < term {
        fib_num = fib(idx, cache);
        if fib_num % 2 == 0 {
            sum += fib_num;
        };
        idx += 1;
    }
    return sum;
}

fn problem_3(mut term: i64) -> i64 {
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

fn main() {
    problem_1();
    let mut cache: LruCache<i64, i64> = LruCache::new(NonZeroUsize::new(2).unwrap());
    println!("{}", problem_2(4000000, &mut cache));
    println!("{}", problem_3(600851475143));
}
