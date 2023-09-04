use std::time::Instant;

mod prime_vec;

fn main() {
    multi2();
    single();
}

fn multi2() {
    let mut primes = prime_vec::PrimeVec::new();
    let begin = Instant::now();
    primes = prime_vec::multi_make_to(primes, 0xffff, 20);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
}


fn single() {
    let begin = Instant::now();
    let primes = prime_vec::PrimeVec::new().simple_make_to(0xffff);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
}


