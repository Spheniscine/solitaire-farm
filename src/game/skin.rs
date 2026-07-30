use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

use crate::game::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum RankSkin {
    #[default]
    #[strum(to_string = "Signed [Sum = 0]")]
    Signed,
    #[strum(to_string = "1~7 [Average = 4]")]
    From1To7,
    #[strum(to_string = "7~13 [Sum ends with 0]")]
    From7To13,
}

impl RankSkin {
    pub fn displayed_rank(self, rank: i8) -> i8 {
        match self {
            RankSkin::Signed => rank,
            RankSkin::From1To7 => rank + 4,
            RankSkin::From7To13 => rank + 10,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default)]
pub enum SuitSkin {
    #[default]
    Default,
}

impl SuitSkin {
    pub fn suit_symbol(self, suit: Suit) -> &'static str {
        match suit {
            Suit::Melon => "🍈",
            Suit::Corn => "🌽",
            Suit::Blueberry => "🫐",
            Suit::Eggplant => "🍆",
        }
    }
}

pub const KATEX_SUITS_FONT_STR: &str = "KaTeX_Suits";

const COLOR_AMBER: [&str; 2] = ["#b70", "#ffb433"];
const COLOR_GREEN: [&str; 2] = ["#062", "#00ff55"];
const COLOR_BLUE: [&str; 2] = ["#00d", "#aaaaff"];
const COLOR_PURPLE: [&str; 2] = ["#60c", "#cf99ff"];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    Dark, #[default] Light
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default)]
pub enum ColorSkin {
    #[default]
    Default,
}

impl ColorSkin {
    pub fn color(self, suit: Suit, mode: ColorMode) -> &'static str {
        let res = match suit {
            Suit::Melon => COLOR_GREEN,
            Suit::Corn => COLOR_AMBER,
            Suit::Blueberry => COLOR_BLUE,
            Suit::Eggplant => COLOR_PURPLE,
        };
        res[mode as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Skin {
    pub ranks: RankSkin,

    #[serde(skip)]
    pub suits: SuitSkin,

    #[serde(skip)]
    pub colors: ColorSkin,
}