use dioxus::{logger::tracing, prelude::*};
use phf::phf_map;

pub static EMOJI_MAP: phf::Map<&'static str, Asset> = phf_map! {
    "🍈" => asset!("/assets/emoji/emoji_u1f348.svg"),
    "🌽" => asset!("/assets/emoji/emoji_u1f33d.svg"),
    "🫐" => asset!("/assets/emoji/emoji_u1fad0.svg"),
    "🍆" => asset!("/assets/emoji/emoji_u1f346.svg"),
    "❌" => asset!("/assets/emoji/emoji_u274c.svg"),
};

#[component]
pub fn Emoji(text: String) -> Element {
    if let Some(asset) = EMOJI_MAP.get(&text) {
        rsx! {
            img {
                style: "height: 1.175em; vertical-align: middle;",
                src: *asset,
                draggable: false,
                alt: text,
            }
        }
    } else {
        tracing::error!("No emoji asset loaded for string '{text}'");
        rsx! {
            "ERROR"
        }
    }
    
}