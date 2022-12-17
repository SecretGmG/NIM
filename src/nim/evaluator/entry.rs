use crate::{nim::generalized::{closed_generalized::ClosedGeneralizedNimGame, GeneralizedNimGame}, vec_ops};

pub (super) struct Entry {
    pub game: ClosedGeneralizedNimGame,
    pub child_games: Option<Vec<Vec<usize>>>,
    pub possible_nimbers: Vec<u16>,
}

impl Entry {
    pub fn new(game: ClosedGeneralizedNimGame) -> Entry {
        return Entry {
            possible_nimbers: vec_ops::ascending_vec_u16(game.get_node_count() + 1),
            game: game,
            child_games: None,
        };
    }
    pub fn get_child_games(&self) -> Vec<GeneralizedNimGame>{
        self.game.get_unique_child_games()
    }
}