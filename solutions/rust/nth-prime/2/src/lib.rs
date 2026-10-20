pub fn nth(n: u32) -> u32 {
    // Sieve of Eratosthenes (not really)
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|c: &u32| {
            if primes.iter().any(|i| c % i == 0) {
                false
            } else {
                primes.push(*c);
                true
            }
        })
        .nth(n as usize)
        .unwrap()
}
