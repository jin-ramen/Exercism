use std::collections::HashSet;

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut primes = HashSet::new();
    let mut i = 2;
    while n > 1 {
        if n % i == 0 {
            n = n / i;
            res.push(i);
            primes.insert(i);
        } else {
            // find the next prime and set i to this prime
            i = ((i+1)..).find(|c: &u64| !primes.iter().any(|&p| c % p == 0)).unwrap();
        }
    }

    res
}
