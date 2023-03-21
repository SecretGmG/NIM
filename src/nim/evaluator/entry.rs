use std::marker::PhantomData;

use super::impartial::*;
use crate::util::vec_ops;

#[derive(Debug)]
pub(super) struct Entry<T, C>
where
    T: ImpartialPart<T, C>,
    C: Impartial<T, C>,
{
    phantom: PhantomData<C>,
    pub game: T,
    pub leaf_indices: Option<Vec<Vec<usize>>>,
    pub possible_nimbers: Vec<u16>,
}

impl<Part, Whole> Entry<Part, Whole>
where
    Part: ImpartialPart<Part, Whole>,
    Whole: Impartial<Part, Whole>,
{
    pub fn new(game: Part) -> Entry<Part, Whole> {
        let easy_nimber = game.get_easy_nimber();
        let possible_nimbers = match easy_nimber {
            Some(nimber) => vec![nimber],
            None => vec_ops::ascending_vec_u16(game.get_max_nimber() + 1),
        };
        return Entry {
            phantom: PhantomData,
            possible_nimbers,
            game: game,
            leaf_indices: None,
        };
    }
    pub fn get_leaf_count(&self) -> usize {
        self.leaf_indices.as_ref().unwrap().len()
    }
    pub fn get_leaf_indices(&self) -> &Vec<Vec<usize>> {
        self.leaf_indices.as_ref().unwrap()
    }
    pub fn set_child_indices(&mut self, child_indices: Vec<Vec<usize>>) {
        self.leaf_indices = Some(child_indices);
    }
    pub fn get_child_games(&self) -> Vec<Whole> {
        self.game.get_unique_leafs()
    }
}
