use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::CardComponent, game::{Card, Skin, Suit}};

#[component]
pub fn Hero() -> Element {
    let position = Vec2::new(10., 10.);
    let width = 12f32;
    let card = Card { rank: 3, suit: Suit::Blueberry };
    let mut skin = Skin::default();
    // skin.ranks = crate::game::RankSkin::From7To13;
    rsx! {
        div {
            id: "hero",

            CardComponent { 
                position,
                width,
                card,
                skin,
            }
        }
    }
}