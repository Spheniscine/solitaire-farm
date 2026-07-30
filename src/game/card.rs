use serde::{Deserialize, Serialize, de::Visitor};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumCount, EnumIter)]
pub enum Suit {
    Melon, Corn, Blueberry, Eggplant
}

impl Suit {
    pub fn code(self) -> char {
        match self {
            Suit::Melon => 'M',
            Suit::Corn => 'C',
            Suit::Blueberry => 'B',
            Suit::Eggplant => 'E',
        }
    }
    pub fn from_code(c: char) -> Option<Self> {
        match c {
            'M' => Some(Suit::Melon),
            'C' => Some(Suit::Corn),
            'B' => Some(Suit::Blueberry),
            'E' => Some(Suit::Eggplant),
            _ => None,
        }
    }
}


impl Serialize for Suit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_char(self.code())
    }
}

impl<'de> Deserialize<'de> for Suit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Suit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "suit code, one of characters MCBE")
            }
            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                where E: serde::de::Error, {
                Suit::from_code(v).ok_or_else(|| E::custom(format!("invalid suit code: {}", v)))
            }
        }
        deserializer.deserialize_char(MyVisitor)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub rank: i8,
    pub suit: Suit, 
}

pub const RANKS_PER_SUIT: [i8; 12] = [-3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3];
pub const NUM_RANKS: usize = RANKS_PER_SUIT.len();
pub const NUM_SUITS: usize = Suit::COUNT;
pub const DECK_SIZE: usize = NUM_RANKS * NUM_SUITS;

impl Card {
    pub fn code(self) -> String {
        format!("{}{}", self.rank, self.suit.code())
    }
    pub fn from_code(code: &str) -> Option<Self> {
        let mut it = code.chars();
        let suit = Suit::from_code(it.next_back()?)?;
        let rank: i8 = it.as_str().parse().ok()?;
        // if !RANKS.contains(&rank) { return None; }
        Some(Card { rank, suit })
    }
}


impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.code())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Card;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "card code")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Card::from_code(v).ok_or_else(|| E::custom(format!("invalid card code: {}", v)))
            }
        }
        deserializer.deserialize_str(MyVisitor)
    }
}


#[cfg(test)]
mod tests {
    use crate::game::{Card, Suit};

    #[test]
    fn card_to_code_test() {
        let card = Card {
            rank: -1, suit: Suit::Melon
        };
        assert_eq!("-1M", card.code())
    }

    #[test]
    fn card_from_code_test() {
        let card = Card {
            rank: -1, suit: Suit::Melon
        };
        assert_eq!(Some(card), Card::from_code("-1M"))
    }
}