use std::time::Instant;
use std::env;

use crate::prime_vec::sqrt;

mod prime_vec;

fn main() {
    let mut args = env::args();

    args.next();

    let primes_to_find :prime_vec::Element = match args.next() {
        Some(arg) => match arg.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("num to find {}, set to 100000",err);
                100000
            },
        },
        None => 100000,
    };

    let worker_count :usize = match args.next() {
        Some(arg) => match arg.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("work thread count {}, set to 0", err);
                0
            }
        },
        None => 0,
    };

    match args.next() {
        Some(_) => { // do find
            println!("multi thread prime find, downward from {primes_to_find}, work thread {worker_count}");
            multi_find(primes_to_find, worker_count);
        },
        None => { // do calc 
            if worker_count > 0 {
                println!("multi thread prime table, upto {primes_to_find}, work thread {worker_count}");
                multi_calc(primes_to_find, worker_count);
            } else {
                println!("single thread prime table, upto {primes_to_find}");
                single(primes_to_find);
            }
        },
    };

}

fn multi_find(primes_to_find :prime_vec::Element, worker_count :usize) {
    let primes_sqrt =  sqrt(primes_to_find); 
    let mut primes = prime_vec::PrimeVec::new_with_cap((primes_sqrt/16) as usize);
    let begin = Instant::now();
    primes = prime_vec::multi_make_to(primes, primes_sqrt, worker_count);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
    for p in (2..=primes_to_find).rev() {
        if primes.is_prime_over(p){
            println!("prime found {p} {primes_to_find}");
            break;
        }
    }
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
    // primes.save();
    // let now = Instant::now();
    // println!("{:?}", now-begin);
}

fn multi_calc(primes_to_find :prime_vec::Element, worker_count :usize) {
    let mut primes = prime_vec::PrimeVec::new_with_cap((primes_to_find/16) as usize);
    let begin = Instant::now();
    primes = prime_vec::multi_make_to(primes, primes_to_find, worker_count);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
    // primes.save();
    // let now = Instant::now();
    // println!("{:?}", now-begin);
}


fn single(primes_to_find :prime_vec::Element) {
    let begin = Instant::now();
    let primes = prime_vec::PrimeVec::new().simple_make_to(primes_to_find);
    let now = Instant::now();
    println!("{} {} {:?}",primes.len(), primes.last(), now-begin);
}


