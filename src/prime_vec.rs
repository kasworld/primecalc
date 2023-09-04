use std::{ thread, sync::{mpsc::{self, Sender}, Arc}};


#[derive(Debug,Clone)]
pub struct PrimeVec(Vec<u64>);

impl PrimeVec {
    pub fn new() -> Self {
        return PrimeVec(vec![2,3,5]) 
    }

    pub fn new_with_cap(n :usize) -> Self {
        let mut rtn = Self::new();
        rtn.0.reserve(n);
        return rtn;
    }

    pub fn simple_make_to(mut self, n :u64) -> Self {
        let mut prime_to_find = self.last()+2;
        while prime_to_find < n {
            if self.is_prime(prime_to_find) {
                self.push(prime_to_find);
            }
            prime_to_find +=2;
        }
        return self;
    }

    pub fn is_prime(&self, prime_to_find :u64)->bool {
        let limit = (prime_to_find as f64).sqrt() as u64;
        if self.last() < limit{
            panic!("too large {} < {limit} == sqrt({prime_to_find})  ", self.last())
        }
        for p in &self.0 {
            if p > &limit {
                return true
            }
            if prime_to_find % p == 0 {
                return false
            }
        }
        return true
    }

    pub fn push(&mut self, prime2append :u64) {
        self.0.push(prime2append)
    }

    pub fn len(&self) -> usize {
        return self.0.len()
    }

    pub fn last(&self) -> u64 {
        return self.0.last().copied().unwrap()
    }

    pub fn append(&mut self, mut from :Vec<u64>) {
        self.0.append(&mut from)
    }


}


pub fn multi_make_to(mut me :PrimeVec, pend :u64, worker_count :usize) -> PrimeVec{
    // println!("{pend}");
    let last = me.last();
    if  last >= pend {
        return me
    }
    if pend > last*last {
        me = multi_make_to(me, pend/2, worker_count);
    }
    let prime_to_find = me.last();
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    let primes = Arc::new(me.clone());

    for wid in 0..worker_count {
        let tx1 = tx.clone();
        let pp = primes.clone();
        let h = thread::spawn(move || {
            worker(pp, tx1, wid, worker_count, prime_to_find, pend)
        });
        handles.push(h);
    }    
    drop(tx);
    let mut rdate: Vec<u64> = Vec::with_capacity( (prime_to_find/16) as usize );
    for r in rx {
        rdate.push(r);
    }
    for h in handles{
        h.join().unwrap()
    }
    rdate.sort_unstable();
    // let mut me = *Arc::get_mut(primes).unwrap();
    me.append(rdate);
    return me
}

fn worker(p :Arc<PrimeVec>, tx :Sender<u64>, wid :usize, wnum :usize, pst :u64, pend :u64) {
    let rst = pst + 2 + 2* wid as u64;
    let rng = (rst .. pend).step_by(2*wnum);
    for i in rng {
        if p.is_prime(i){
            tx.send(i).unwrap();
        }
    }
}
