use std::{ thread, sync::{mpsc::{self,  Sender}, Arc}, time::Instant};

use prime_vec::PrimeVec;

mod prime_vec;

fn main() {
    multi()
}

fn worker(p :Arc<PrimeVec>, tx : Sender<u64>, wid :usize,wnum :usize, pst :u64, pend :u64) {
    let rst = pst + 2 + 2* wid as u64;
    let rng = (rst .. pend).step_by(2*wnum);
    for i in rng {
        if p.is_prime(i){
            tx.send(i).unwrap();
        }
    }
}

fn multi() {
    let mut primes = prime_vec::PrimeVec::new();
    primes.simple_make_to(0xff);
    let prime_to_find = primes.last();
    let worker_count :usize = 2;
    let pend = 0xfff;

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    let mut primes = Arc::new(primes);

    let begin = Instant::now();

    for wid in 0..worker_count {
        let tx1 = tx.clone();
        let pp = primes.clone();
        let h = thread::spawn(move || 
            worker(pp, tx1, wid,worker_count, prime_to_find, pend)
        );
        handles.push(h);
    }    
    drop(tx);
    let mut rdate  = Vec::new();
    for r in rx {
        rdate.push(r);
        // print!("{r} ");
    }
    for h in handles{
        h.join().unwrap()
    }
    rdate.sort_unstable();
    let primes = Arc::get_mut(&mut primes).unwrap();
    primes.append(rdate);

    // println!("{pend} {} {} ", primes.len(), primes.last());
    println!("{:?}", primes);
    let now = Instant::now();
    println!("{:?}", now-begin);
}


// fn single() {
//     let mut primes = prime_vec::PrimeVec::new();
//     let mut prime_to_find = primes.last();
//     let begin = Instant::now();
//     loop {
//         prime_to_find += 2;
//         if primes.is_prime(prime_to_find) {
//             primes.append(prime_to_find);
//             if primes.len() % 10000 == 0 {
//                 let now = Instant::now();
//                 println!("{prime_to_find} {} {:?}", primes.len(), now - begin);
//             }
//         }
//     }
// }
