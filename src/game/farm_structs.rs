#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct BitMask(pub u16);
impl Iterator for BitMask {
    type Item = usize;
 
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 { return None; }
        let r = self.0.trailing_zeros();
        self.0 &= self.0 - 1;
        Some(r as usize)
    }
}
impl BitMask {
    pub fn single(x: usize) -> Self {
        Self(1 << x)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, x: usize) -> bool {
        self.0 >> x & 1 == 1
    }

    pub fn flip(self, x: usize) -> Self {
        Self(self.0 ^ (1 << x))
    }
}

pub const FARM_HEIGHT: usize = 3;
pub const FARM_WIDTH: usize = 3;
pub const NUM_FARM_PLOTS: usize = FARM_HEIGHT * FARM_WIDTH;

/// to row-column coordinates
pub fn to_farm_coords(x: usize) -> (usize, usize) {
    assert!(x < NUM_FARM_PLOTS);
    (x / FARM_WIDTH, x % FARM_WIDTH)
}

/// from row-column coordinates
pub fn from_farm_coords(r: usize, c: usize) -> usize {
    assert!(r < FARM_HEIGHT && c < FARM_WIDTH);
    r * FARM_WIDTH + c
}
