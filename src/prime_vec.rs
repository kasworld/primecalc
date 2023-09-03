#[derive(Debug,Clone)]
pub struct PrimeVec(Vec<u64>);

impl PrimeVec {
    pub fn new() -> Self {
        return PrimeVec(vec![2,3,5]) 
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

    pub fn simple_make_to(&mut self, n :u64) {
        let mut prime_to_find = self.last()+2;
        while prime_to_find < n {
            if self.is_prime(prime_to_find) {
                self.push(prime_to_find);
            }
            prime_to_find +=2;
        }
    }
}
