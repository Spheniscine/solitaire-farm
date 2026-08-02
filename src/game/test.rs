use phf::{Set, phf_set};

use crate::game::{BitMask, Suit};

const TEST_SET: Set<u16> = phf_set! { 0u16, 1 };

pub fn is_crop_group_shape(suit: Suit, mask: BitMask) -> bool {
    match suit {
        Suit::Melon => TEST_SET.contains(&mask.0),
        Suit::Corn => TEST_SET.contains(&mask.0),
        Suit::Blueberry => TEST_SET.contains(&mask.0),
        Suit::Eggplant => TEST_SET.contains(&mask.0),
    }
} 