use std::env;
use std::time::Instant;

use crate::prime_vec::sqrt;

mod prime_vec;

fn main() {
    let mut args = env::args();

    let prgname =  args.next().unwrap();

    let primes_to_find: prime_vec::Element = match args.next() {
        Some(arg) => match arg.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("num to find {}, set to 100000", err);
                100000
            }
        },
        None => 100000,
    };

    let worker_count = prime_vec::get_thread_count() - 1;
    match args.next() {
        Some(arg) => {
            match arg.trim() {
                "single" => {
                    println!("single thread make prime table, upto {primes_to_find}");
                    single(primes_to_find);
                }
                "find" => {
                    // do find
                    println!("multi thread prime find, downward from {primes_to_find}, work thread {worker_count}");
                    multi_find(primes_to_find, worker_count);
                }
                "calc" => {
                    println!(
                        "multi thread make prime table, upto {primes_to_find}, work thread {worker_count}"
                    );
                    multi_calc(primes_to_find, worker_count);
                }
                _ => {
                    // print help
                    help(prgname);
                }
            }
        }
        None => {
            help(prgname);
        }
    };
}

fn help(prgname: String) {
    println!("{prgname} prime number find, calc, data save, load");
    println!("{prgname} number cmd");
    println!("number : find downward or make table to number");
    println!("cmd : single : single thread make prime table upto number");
    println!("cmd : calc   : multi thread make prime table upto number");
    println!("cmd : find   : multi thread find prime downward from number, make table if need");

}

fn multi_find(primes_to_find: prime_vec::Element, worker_count: usize) {
    let primes_sqrt = sqrt(primes_to_find);
    let mut primes = prime_vec::PrimeVec::new_with_cap((primes_sqrt / 16) as usize);

    let begin = Instant::now();

    primes.load();
    println!(
        "after load {} {:?}",
        prime_vec::get_filename(),
        Instant::now() - begin
    );

    primes = prime_vec::multi_make_to(primes, primes_sqrt, worker_count);
    println!(
        "after multi_make_to {} {} {:?}",
        primes.len(),
        primes.last(),
        Instant::now() - begin
    );

    for p in (2..=primes_to_find).rev() {
        if primes.is_prime_over(p) {
            println!("prime found {p} {primes_to_find}");
            break;
        }
    }
    println!(
        "after search {} {} {:?}",
        primes.len(),
        primes.last(),
        Instant::now() - begin
    );

    primes.save();
    println!(
        "after save {} {:?}",
        prime_vec::get_filename(),
        Instant::now() - begin
    );
}

fn multi_calc(primes_to_find: prime_vec::Element, worker_count: usize) {
    let mut primes = prime_vec::PrimeVec::new_with_cap((primes_to_find / 16) as usize);

    let begin = Instant::now();

    primes.load();
    println!(
        "after load {} {:?}",
        prime_vec::get_filename(),
        Instant::now() - begin
    );

    primes = prime_vec::multi_make_to(primes, primes_to_find, worker_count);
    println!(
        "after multi_make_to {} {} {:?}",
        primes.len(),
        primes.last(),
        Instant::now() - begin
    );

    primes.save();
    println!(
        "after save {} {:?}",
        prime_vec::get_filename(),
        Instant::now() - begin
    );
}

fn single(primes_to_find: prime_vec::Element) {
    let mut primes = prime_vec::PrimeVec::new();
    let begin = Instant::now();

    primes.load();
    println!(
        "after load {} {:?}",
        prime_vec::get_filename(),
        Instant::now() - begin
    );

    primes = primes.simple_make_to(primes_to_find);
    println!(
        "after simple_make_to {} {} {:?}",
        primes.len(),
        primes.last(),
        Instant::now() - begin
    );

    primes.save();
    println!(
        "after save {} {:?}",
        prime_vec::get_filename(),
        Instant::now() - begin
    );
}
