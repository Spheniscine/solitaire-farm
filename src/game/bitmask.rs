use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct BitMask(u16);
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
    pub fn contains(self, x: usize) -> bool {
        self.0 >> x & 1 == 1
    }

    pub fn flip(self, x: usize) -> Self {
        Self(self.0 ^ (1 << x))
    }
}