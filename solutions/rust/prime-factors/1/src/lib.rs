use std::collections::HashSet;

pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut primes = HashSet::new();
    let mut x = n;
    let mut i = 2;
    loop {
        if x == 1 {
            break;
        }

        if x % i == 0 {
            x = x / i;
            res.push(i);
            primes.insert(i);
        } else {
            // find the next prime and set i to this prime
            i = ((i+1)..).find(|c: &u64| !primes.iter().any(|&p| c % p == 0)).unwrap();
        }
    }

    res
}
