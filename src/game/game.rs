use std::time::Duration;

use dioxus::logger::tracing;
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{BitMask, Board, BoardPos, Card, DECK_SIZE, DepotRole, RANKS_PER_SUIT, Selection, Skin, Suit, is_crop_group_shape}};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS_PER_SUIT {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        LocalStorage.save_game_state(&self);
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn is_won(&self) -> bool {
        use DepotRole::*;
        self.board.depots[Market.id(0)].len() == DECK_SIZE
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.board.do_move(pos1, pos2);
        self.history.push(ActionRecord { pos1, pos2 })
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn valid_crop_group_selected(&self) -> bool {
        let Some(Selection::Farm(mask)) = self.board.selected else {return false};
        let cards_iter = || mask.map(|i| self.board.depots[DepotRole::Farm.id(i)].last());
        let mut iter = cards_iter();
        let Some(Some(&first_card)) = iter.next() else {return false};
        iter.all(|card| card.is_some_and(|card| card.suit == first_card.suit)) &&
            is_crop_group_shape(first_card.suit, mask) &&
            cards_iter().map(|card| card.unwrap().rank).sum::<i8>() == 0
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        back.suit != front.suit && front.rank + 1 == back.rank
    }

    fn try_select(&mut self, pos: BoardPos) {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some((role, index)) = DepotRole::role_and_subindex(depot) else { return };
        match role {
            DepotRole::Tableau => if slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
                {self.board.selected = Some(Selection::Stack(pos));},
            DepotRole::FreeCell => if slice.len() == 1 {self.board.selected = Some(Selection::Stack(pos));},
            DepotRole::Farm => if slice.len() == 1 {self.board.selected = Some(Selection::Farm(BitMask::single(index)))},
            DepotRole::Market => return,
        }
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }

        if let Some(selected) = self.board.selected {
            match selected {
                Selection::Stack(src) => {
                    if pos == src { 
                        self.board.selected = None; 
                        return;
                    }
                    if src.depot_index == pos.depot_index {
                        self.try_select(pos);
                        return;
                    }

                    let dest = BoardPos::new(pos.depot_index, pos.card_index.wrapping_add(1));
                    self.move_intent(src, dest);
                },
                Selection::Farm(mut mask) => {
                    let depot = pos.depot_index;

                    let Some((role, index)) = DepotRole::role_and_subindex(depot) else { return };
                    match role {
                        DepotRole::Farm => {
                            if self.board.depots[depot].is_empty() { return; }
                            mask = mask.flip(index);
                            if mask.is_empty() {
                                self.board.selected = None;
                            } else {
                                self.board.selected = Some(Selection::Farm(mask))
                            }
                        },
                        DepotRole::Market => {
                            if self.valid_crop_group_selected() {
                                let history_len = self.history.len();
                                let mut market_pos = self.board.top_pos(DepotRole::Market.id(0));
                                for i in mask {
                                    self.do_move_raw(BoardPos::new(DepotRole::Farm.id(i), 0), 
                                        market_pos);
                                    market_pos.card_index += 1;
                                }
                                self.undo_stack.push(history_len);
                            }
                        },
                        _ => {}
                    }
                },
            }
        } else {
            self.try_select(pos);
        }
    }

    pub fn ondoubleclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }

        if DepotRole::role(pos.depot_index) != Some(DepotRole::Tableau) { return; }
        let depot = &self.board.depots[pos.depot_index];
        if pos.card_index >= depot.len() { return; }
        self.move_intent(pos, self.board.top_pos(DepotRole::FreeCell.id(0)));
    }

    fn move_intent(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];
        let Some(role) = DepotRole::role(pos2.depot_index) else { return false };

        let history_len = self.history.len();
        match role {
            DepotRole::Tableau => {
                let ok = depot2.last().is_none_or(|&c| self.can_stack(c, card));
                if !ok { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::FreeCell => {
                if num_moved != 1 || !depot2.is_empty() { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::Farm => {
                if num_moved != 1 || !depot2.is_empty() { return false; }
                self.do_move_raw(pos1, pos2);
            },
            DepotRole::Market => return false,
        }

        self.undo_stack.push(history_len);
        true
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            self.board.do_move(rec.pos2, rec.pos1);
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn clear_selection(&mut self) {
        self.board.selected = None;
    }
}