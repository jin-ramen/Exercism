pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut i = 3;
    loop {
        if primes.len() == (n + 1) as usize {
            break;
        }

        let mut is_prime = true;
        for p in &primes {
            if i % p == 0 {
                is_prime = false;
            }
        }

        if is_prime {
            primes.push(i);
        }
        i += 1;
    }
    primes[primes.len() - 1]
}
