use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_BORDER_RADIUS_RATIO, CARD_HEIGHT_RATIO, CardComponent, CardFrame, Movement, rem}, game::{AnimationAct, AnimationKey, Board, BoardPos, Card, DepotRole, FARM_HEIGHT, FARM_WIDTH, NUM_DEPOTS, Selection, Skin, to_farm_coords}};

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    ondoubleclick: EventHandler<BoardPos>,
    #[props(default)]
    oncontextmenu: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
    #[props(default)]
    is_won: bool,
    #[props(default)]
    valid_crop_group_selected: bool,
) -> Element {
    let card_width = 12f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1f32;
    let spacer_y = 1f32;
    let start_x = 2f32;
    let start_y = 2f32;

    let pos_x = {
        move |i: usize| {
            start_x + (card_width + spacer_x) * i as f32
        }
    };
    let tableau_y = start_y + card_height + spacer_y;

    let market_x = 100. - start_x - card_width;
    
    let farm_w = (card_width + spacer_x) * FARM_WIDTH as f32 + spacer_x;
    let farm_h = (card_height + spacer_y) * FARM_HEIGHT as f32 + spacer_y;
    let farm_pos = Vec2::new(
        100. - start_x - farm_w,
        tableau_y + spacer_y
    );

    let column_card_offset = Vec2::new(0., 6.5);

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        match role {
            DepotRole::Tableau => 
                Vec2::new(pos_x(index), tableau_y) + column_card_offset * ord as f32,
            DepotRole::FreeCell => 
                Vec2::new(start_x, start_y),
            DepotRole::Farm => {
                let (i, j) = to_farm_coords(index);
                Vec2::new(farm_pos.x + spacer_x + (card_width + spacer_x) * i as f32,
                    farm_pos.y + spacer_y + (card_height + spacer_y) * j as f32)
            },
            DepotRole::Market => 
                Vec2::new(market_x, start_y),
        }
    };

    let symbol2 = |text: &str| rsx! {
        span {
            font_family: "'Noto Sans Symbols 2'",
            position: "relative",
            top: "0.12em",
            {text}
        }
    };

    let get_hint = |depot: usize| {
        let role = DepotRole::role(depot).unwrap();
        match role {
            DepotRole::Tableau => Some(rsx!{}),
            DepotRole::FreeCell => Some(symbol2("✽")),
            DepotRole::Farm => Some(rsx!{}),
            DepotRole::Market => 
                Some(
                    rsx!{
                        span {
                            font_family: "'Noto Emoji'",
                            "💰"
                        }
                    }
                ),
        }
    };

    let is_face_up = |depot: usize| {
        DepotRole::role(depot).unwrap().is_face_up()
    };

    let selected_height = match board.selected {
        Some(Selection::Stack(BoardPos { depot_index, card_index })) => {
            let d = if DepotRole::role(depot_index).unwrap() == DepotRole::Tableau {
                board.depots[depot_index].len() - card_index - 1
            } else {
                0
            };

            card_height + column_card_offset.y * d as f32
        },
        Some(Selection::Farm(_)) => card_height,
        None => 0.,
    };

    let is_selected = |pos: BoardPos| {
        match board.selected {
            Some(Selection::Stack(pos2)) => pos == pos2,
            Some(Selection::Farm(mask)) => {
                let (role, index) = DepotRole::role_and_subindex(pos.depot_index).unwrap();
                role == DepotRole::Farm && mask.contains(index)
            },
            None => false,
        }
    };

    let selection_color = if valid_crop_group_selected {"#0f0"} else {"#ff0"};

    let moving_card = |p1: Vec2, p2: Vec2, card: Card| rsx! {
        Movement {
            src_translate_vec: p1 - p2,
            CardComponent {
                position: p2,
                width: card_width,
                card: card,
                skin,
            }
        }
    };

    let anims = board.animation_acts.iter().enumerate().map(|(i, act)| {
        match act {
            AnimationAct::Move (cards, pos1, pos2) => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;

                let nodes = cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = moving_card(p1, p2, *card);
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                });

                rsx! {
                    Fragment {
                        key: "{animation_key},{i}", // needed to force remounts, so animations don't get "stale" and refuse to replay
                        {nodes}
                    }
                }
            },
        }
    });

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            // farm soil decor
            div {
                position: "absolute",
                top: rem(farm_pos.y),
                left: rem(farm_pos.x),
                width: rem(farm_w),
                height: rem(farm_h),
                background_color: "#530",
                border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
            },

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, !0))
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    if is_selected(BoardPos::new(depot, i)) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: selection_color,
                            border_radius: rem(card_width * CARD_BORDER_RADIUS_RATIO),
                            class: "selected-halo",
                        }
                    }

                    
                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: if is_face_up(depot) {board.depots[depot][i]},
                        // number_hint: if !is_face_up(depot) {i + 1},
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos::new(depot, i))
                        },
                        ondoubleclick: move |_| {
                            ondoubleclick.call(BoardPos::new(depot, i))
                        },
                    }
                }
            }

            {anims}

            if is_won {
                div {
                    position: "absolute",
                    top: rem(25.),
                    left: rem(17.5),
                    width: rem(59.),
                    background_color: "#505",
                    padding: rem(3.),
                    color: "#fff",
                    font_size: rem(7.),
                    border_radius: rem(2.),
                    text_align: "center",
                    "YOU WIN!",
                }
            }
        }
    }
}