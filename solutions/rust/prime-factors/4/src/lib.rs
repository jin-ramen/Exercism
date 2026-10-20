pub fn factors(mut n: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut f = 2;
    while n > 1 {
        while n % f == 0 {
            primes.push(f);
            n /= f;
        }
        f += 1;
    }

    primes
}
