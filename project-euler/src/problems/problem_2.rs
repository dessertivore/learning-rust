use std::num::NonZeroUsize;

use lru::LruCache;

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

pub fn problem_2(term: i64) -> i64 {
    let mut cache: LruCache<i64, i64> = LruCache::new(NonZeroUsize::new(2).unwrap());
    let mut sum = 0;
    let mut fib_num = 0;
    let mut idx = 0;
    while fib_num < term {
        fib_num = fib(idx, &mut cache);
        if fib_num % 2 == 0 {
            sum += fib_num;
        };
        idx += 1;
    }
    return sum;
}
