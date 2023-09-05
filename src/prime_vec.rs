use std::{ thread, sync::{mpsc::{self, Sender}, Arc}};

pub type Element = u64;

#[derive(Debug,Clone)]
pub struct PrimeVec(Vec<Element>);

impl PrimeVec {
    pub fn new() -> Self {
        return PrimeVec(vec![2,3,5]) 
    }

    pub fn new_with_cap(n :usize) -> Self {
        let mut rtn = Self::new();
        rtn.0.reserve(n);
        return rtn;
    }

    pub fn simple_make_to(mut self, n :Element) -> Self {
        let mut prime_to_find = self.last()+2;
        while prime_to_find <= n {
            if self.is_prime(prime_to_find) {
                self.push(prime_to_find);
            }
            prime_to_find +=2;
        }
        return self;
    }

    // in table
    pub fn is_prime(&self, prime_to_find :Element)->bool {
        let limit = sqrt(prime_to_find);
        let last = self.last();
        if last < limit{
            panic!("too large {last} < {limit} == sqrt({prime_to_find})")
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

    // over table 
    pub fn is_prime_over(&self, prime_to_find :Element)->bool {
        let limit = sqrt(prime_to_find);
        let last_prime_can_find = self.last_prime_can_find();
        if last_prime_can_find < limit{
            panic!("too large {last_prime_can_find} < {limit} == sqrt({prime_to_find})")
        }
        for p in &self.0 {
            if p > &limit {
                return true
            }
            if prime_to_find % p == 0 {
                return false
            }
        }
        let last = self.last();
        for p in (last+2..=limit).step_by(2) {
            if prime_to_find % p == 0 {
                return false
            }
        }
        return true
    }

    pub fn push(&mut self, prime2append :Element) {
        self.0.push(prime2append)
    }

    pub fn len(&self) -> usize {
        return self.0.len()
    }

    pub fn last(&self) -> Element {
        return self.0.last().copied().unwrap()
    }
    
    pub fn last_prime_can_find(&self) -> Element {
        let last = self.last();
        return last*last;
    }

    pub fn append(&mut self, mut from :Vec<Element>) {
        self.0.append(&mut from)
    }


}


pub fn sqrt(v :Element)->Element{
    return (v as f64).sqrt() as Element
}

pub fn multi_make_to(mut me :PrimeVec, pend :Element, thread_count :usize) -> PrimeVec{
    // println!("{pend}");
    let last = me.last();
    if  last >= pend {
        return me
    }
    if pend > me.last_prime_can_find() {
        me = multi_make_to(me, pend/2, thread_count);
    }
    let prime_to_find = me.last();
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    let primes = Arc::new(me.clone());

    for wid in 0..thread_count {
        let tx1 = tx.clone();
        let pp = primes.clone();
        let h = thread::spawn(move || {
            worker(pp, tx1, wid, thread_count, prime_to_find, pend)
        });
        handles.push(h);
    }    
    drop(tx);
    let mut rdate: Vec<Element> = Vec::with_capacity( (prime_to_find/16) as usize );
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

fn worker(p :Arc<PrimeVec>, tx :Sender<Element>, wid :usize, wnum :usize, pst :Element, pend :Element) {
    let rst = pst + 2 + 2* wid as Element;
    let rng = (rst .. pend).step_by(2*wnum);
    for i in rng {
        if p.is_prime(i){
            tx.send(i).unwrap();
        }
    }
}

// no table use 
pub fn is_prime(prime_to_find :Element)->bool {
    if prime_to_find % 2 == 0 {
        return prime_to_find == 2;
    }
    let limit = sqrt(prime_to_find);
    for p in (3..=limit).step_by(2) {
        if prime_to_find % p == 0 {
            return false
        }
    }
    return true
}

