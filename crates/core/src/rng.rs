//! Seeded chance (D2): the seed comes in from the page once, at a new game,
//! and every roll after that is a function of the state. The same save and the
//! same elapsed milliseconds give the same result on every machine.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rng {
    pub s: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { s: seed ^ 0x9E37_79B9_7F4A_7C15 | 1 }
    }

    /// xorshift64*.
    pub fn next(&mut self) -> u64 {
        let mut x = self.s;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.s = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A whole number in `0..n` (`n` > 0).
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }

    /// True `pct` times in a hundred.
    pub fn percent(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }

    /// True once in `n` on average.
    pub fn one_in(&mut self, n: u64) -> bool {
        self.below(n) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_seed_gives_the_same_rolls() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        let ra: Vec<u64> = (0..5).map(|_| a.below(1000)).collect();
        let rb: Vec<u64> = (0..5).map(|_| b.below(1000)).collect();
        assert_eq!(ra, rb);
    }

    #[test]
    fn a_zero_seed_still_rolls() {
        let mut r = Rng::new(0x9E37_79B9_7F4A_7C15);
        assert_ne!(r.next(), 0);
    }

    #[test]
    fn percent_is_near_its_rate_over_many_rolls() {
        let mut r = Rng::new(7);
        let hits = (0..10_000).filter(|_| r.percent(25)).count();
        assert!((2_300..2_700).contains(&hits), "{hits}");
    }
}
