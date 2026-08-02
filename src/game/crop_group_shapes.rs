use phf::{Set, phf_set};

use crate::game::{BitMask, Suit};

include!(concat!(env!("OUT_DIR"), "/crop_group_shapes.rs"));

pub fn is_crop_group_shape(suit: Suit, mask: BitMask) -> bool {
    match suit {
        Suit::Melon => MELON_GROUPS.contains(&mask.0),
        Suit::Corn => CORN_GROUPS.contains(&mask.0),
        Suit::Blueberry => BLUEBERRY_GROUPS.contains(&mask.0),
        Suit::Eggplant => EGGPLANT_GROUPS.contains(&mask.0),
    }
}