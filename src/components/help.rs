use dioxus::prelude::*;

use crate::{components::{CardText, Emoji, SkinTrait, VIDEO_GAMEPLAY, rem}, game::{Card, ColorMode, GameState, RankSkin, ScreenState, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let stack_example = || {
        let mut ite = [
            Card { rank: 2, suit: Suit::Corn },
            Card { rank: 1, suit: Suit::Melon },
            Card { rank: 0, suit: Suit::Eggplant },
            Card { rank: -1, suit: Suit::Melon },
            Card { rank: -2, suit: Suit::Blueberry },
        ].into_iter().map(|card| {
            rsx! {
                CardText { 
                    card, skin, color_mode: ColorMode::Light,
                }
            }
        });


        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { ",", {x} },
            " can be placed on ", {last}
        }
    };

    let rank_text = |rank: i8| {
        rsx! {
            span {
                font_size: "1.2em",
                {skin.render_rank(&Card { rank, suit: Suit::Melon })}
            }
        }
    };

    let suit_text = |suit: Suit| {
        let card = Card { rank: 0, suit };
        rsx! {
            {skin.render_suit(&card)}
            span {
                color: skin.get_color(&card, ColorMode::Light),
                " {suit}",
            }
        }
    };

    let sum_rule = match skin.ranks {
        RankSkin::Signed => "a sum of 0",
        RankSkin::From1To7 => "an average of exactly 4",
        RankSkin::From7To13 => "a sum ending in 0",
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 3.35rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a special 48-card deck. There are 4 suits representing types of crops, and each suit has:"
                    ul {
                        li { "1 copy each of ranks ", {rank_text(-3)}, " and ", {rank_text(3)}},

                        li { "2 copies each of ranks ", {rank_text(-2)}, " through ", {rank_text(2)}},
                    }
                }

                p {
                    "Cards stack in the ", Emph{"tableau"}, " by ", Emph {"descending rank"}, " and " Emph {"unlike suit"},
                    ". Such stacks of any size can be moved as a unit. (e.g. ",{stack_example()},")."
                }

                p {
                    "The ",Emph{"free cell"}," on the top-left may store a single card of any kind."
                }

                p {
                    "The ",Emph{"farm"}," consists of 9 plots in a 3×3 grid. Each plot may hold 1 card. Cards may not be removed from 
                    plots until harvested."
                }

                p {
                    "You may ", Emph{"harvest"}, " cards if they form a group that satisfies these conditions:"
                    ul {
                        li { "The group must be of the ", Emph{"same suit"}, ", and its ranks must have ", Emph{{sum_rule}}, ".", },

                        li { "The group must be arranged in a certain shape, depending on its suit:",
                            br{}, span {
                                {suit_text(Suit::Melon)},
                                ": 4 contiguous plots in a 2×2 square."
                            },
                            br{}, span {
                                {suit_text(Suit::Corn)},
                                ": 2 or 3 contiguous plots in a vertical rectangle."
                            },
                            br{}, span {
                                {suit_text(Suit::Blueberry)},
                                ": 2 non-contiguous plots. (Diagonals are allowed.)"
                            },
                            br{}, span {
                                {suit_text(Suit::Eggplant)},
                                ": 3 contiguous plots in an L shape. Rotations are allowed."
                            },
                        },

                    }
                }

                p {
                    "To harvest cards, select all the cards in the farm that form a valid group, then click on the ", 
                    Emph{"market"}, ". The ", Emoji { text: "❌" }, " button deselects all cards."
                }

                p {
                    "To ",Emph{"win the game"},", you must harvest all the cards."
                }
            }

            div {
                position: "absolute",
                bottom: rem(2.),
                width: "92rem",
                display: "flex",
                justify_content: "center",

                a {
                    href: VIDEO_GAMEPLAY,
                    target: "_blank",
                    text_decoration: "none",
                    margin_right: rem(4.),
                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        "Example video"
                    }
                }

                div {
                    width: rem(30.),
                    position: "relative",
                    class: "game-button",
                    onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                    "Back to game"
                }
            }
        }
    }
}