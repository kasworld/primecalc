use std::time::Instant;
use std::env;

mod prime_vec;

fn main() {
    let mut args = env::args();

    args.next();

    let primes_to_find :u64 = match args.next() {
        Some(arg) => match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => 0xffff,
        },
        None => 0xffff,
    };

    let worker_count :usize = match args.next() {
        Some(arg) => match arg.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        },
        None => 0,
    };
    if worker_count > 0 {
        println!("multi thread prime to {primes_to_find} worker {worker_count}");
        multi2(primes_to_find,worker_count);
    } else {
        println!("single thread prime to {primes_to_find}");
        single(primes_to_find);
    }
}

fn multi2(primes_to_find :u64, worker_count :usize) {
    let mut primes = prime_vec::PrimeVec::new();
    let begin = Instant::now();
    primes = prime_vec::multi_make_to(primes, primes_to_find, worker_count);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
}


fn single(primes_to_find :u64) {
    let begin = Instant::now();
    let primes = prime_vec::PrimeVec::new().simple_make_to(primes_to_find);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
}


