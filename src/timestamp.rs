use super::precision::*;
use std::ops::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Timestamp(pub(crate) u64);

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elapsed(u64);

impl Sub for Timestamp {
    type Output = Elapsed;

    #[inline]
    fn sub(self, ts: Timestamp) -> Self::Output {
        if self.0 >= ts.0 {
            Elapsed(self.0 - ts.0)
        } else {
            Elapsed(self.0 + (!ts.0) + 1)
        }
    }
}

impl Add for Elapsed {
    type Output = Elapsed;

    #[inline]
    fn add(self, other: Elapsed) -> Self::Output {
        Elapsed(self.0 + other.0)
    }
}

impl AddAssign for Elapsed {
    #[inline]
    fn add_assign(&mut self, other: Elapsed) {
        self.0 += other.0;
    }
}

impl Elapsed {
    #[inline]
    pub fn new() -> Self {
        Elapsed(0)
    }

    #[inline]
    pub fn from_ticks(ticks: u64) -> Self {
        Elapsed(ticks)
    }

    #[inline]
    pub fn ticks(&self) -> u64 {
        self.0
    }

    #[inline]
    pub fn as_secs(&self, precision: &Precision) -> u64 {
        self.0 / precision.frequency
    }

    #[inline]
    pub fn as_secs_f64(&self, precision: &Precision) -> f64 {
        self.0 as f64 / precision.frequency as f64
    }

    #[inline]
    pub fn as_millis(&self, precision: &Precision) -> u64 {
        self.0 * 1_000 / precision.frequency
    }

    #[inline]
    pub fn as_ns(&self, precision: &Precision) -> u64 {
        (((self.0 as f64 * 1_000.0) / precision.frequency as f64) * 1_000_000.0) as u64
    }
}
