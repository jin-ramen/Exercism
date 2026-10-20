pub fn factors(mut n: u64) -> Vec<u64> {
    let mut primes = vec![];

    while n > 1 {
        let i = (2..=n).find(|p| n % p == 0).unwrap();
        primes.push(i);
        n /= i;
    }

    primes
}
