use dioxus::prelude::*;

use crate::{components::{Emoji, SkinTrait}, game::{Card, ColorMode, KATEX_SUITS_FONT_STR, RankSkin, Skin}};

impl Skin {
    fn render_suit_internal(&self, card: &Card, _text_mode: bool) -> Element {
        rsx! {
            Emoji { 
                text: self.suits.suit_symbol(card.suit)
            }
        }
    }
}


impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card, mode: ColorMode) -> String {
        self.colors.color(card.suit, mode).to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        let rank = self.ranks.displayed_rank(card.rank);
        if self.ranks == RankSkin::Signed {
            rsx! {
                if rank != 0 {
                    span {
                        font_family: "'Noto Sans'", // KaTeX sign symbols are too large
                    if rank > 0 {"+"} else {"‒"} // "figure dash is used, as it renders to match the plus sign"
                    }
                }
                span {
                    font_family: KATEX_SUITS_FONT_STR,
                    "{rank.abs()}"
                }
            }
        } else {
            rsx! {
                span {
                    font_family: KATEX_SUITS_FONT_STR,
                    "{rank}"
                }
            }
        }
    }

    fn render_suit(&self, card: &Card) -> Element {
        self.render_suit_internal(card, false)
    }

    fn render_suit_text(&self, card: &Card) -> Element {
        self.render_suit_internal(card, true)
    }
}