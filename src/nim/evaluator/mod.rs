mod entry;
pub mod impartial;
mod implementations;
use entry::Entry;
use impartial::*;
use std::{collections::HashMap, marker::PhantomData};

/// Evaluates an impartial game
/// The generic arguments specify
/// a generalized version and a smaller part of a generalized impartial game
pub struct Evaluator<Part, Whole>
where
    Part: ImpartialPart<Part, Whole>,
    Whole: Impartial<Part, Whole>,
{
    data: Vec<Entry<Part, Whole>>,
    indecies: HashMap<Part, usize>,
    phantom: PhantomData<Whole>,
}

impl<Part, Whole> Evaluator<Part, Whole>
where
    Part: ImpartialPart<Part, Whole>,
    Whole: Impartial<Part, Whole>,
{
    pub fn new() -> Evaluator<Part, Whole> {
        Evaluator {
            phantom: PhantomData,
            data: vec![],
            indecies: HashMap::new(),
        }
    }
    ///calculates the nimber of an impartial game
    pub fn get_nimber_of_whole(&mut self, g: &Whole) -> u16 {
        return self
            .get_nimber_of_whole_bounded(g, u16::max_value())
            .unwrap();
    }
    #[allow(dead_code)]
    ///calculates the nimber of an impartial game but stoppes if the evaluator
    ///is certain that the nimber of the game is above the bound
    pub fn get_nimber_of_whole_bounded(&mut self, g: &Whole, bound: u16) -> Option<u16> {
        let parts_indices = self.create_indecies_of(g);
        return self.get_nimber_of_indices_bounded(&parts_indices, bound);
    }
    ///gets bounded nimber given an index
    fn get_nimber_of_index_bounded(&mut self, index: usize, bound: u16) -> Option<u16> {
        //test if there is a child game for which child_nimber < nimber
        loop {
            let entry = self.get(index); //reborrow the entry

            //if there is only one possible nimber left, return it
            if entry.possible_nimbers.len() == 1 {
                return Some(entry.possible_nimbers[0]);
            }
            let smallest_possible_nimber = *entry.possible_nimbers.first().unwrap();
            //if the nimber of self.get(index) definitly is bigger than the bound return
            if smallest_possible_nimber > bound {
                return None;
            }
            //if the self.get(index) has no child with the smallest_possible_nimber if self.get(index)
            //then self.get(index) must have the nimber smallest_possible_nimber
            if !self.has_leaf_with_nimber(index, smallest_possible_nimber) {
                self.set_nimber(index, smallest_possible_nimber);
                return Some(smallest_possible_nimber);
            }
        }
    }
    fn get_nimber_of_indices_bounded(&mut self, indices: &Vec<usize>, nimber: u16) -> Option<u16> {
        let mut modifier = 0;

        if indices.len() == 0 {
            return Some(0);
        }
        //accumulate all the nimbers of the first (n-1) parts
        for part_index in 0..(indices.len() - 1) {
            modifier ^= self
                .get_nimber_of_index_bounded(indices[part_index], u16::MAX)
                .unwrap();
        }
        //index of the last part of the current child game
        let last_part = indices.last().unwrap();
        //if the last part has the _nimber == nimber xor modifier
        self.get_nimber_of_index_bounded(*last_part, nimber ^ modifier)
    }

    ///checks if self.get(index) has a leaf with a given nimber
    fn has_leaf_with_nimber(&mut self, index: usize, nimber: u16) -> bool {
        self.generate_leaf_entries(index);

        let leaf_count = self.get(index).get_leaf_count();
        //if the node has no leafs return;
        if leaf_count == 0 {
            self.set_nimber(index, 0);
            return nimber == 0;
        }

        for leaf_indices in self.get(index).get_leaf_indices().clone() {
            match self.get_nimber_of_indices_bounded(&leaf_indices, nimber) {
                Some(leaf_nimber) => {
                    self.remove_nimber(index, leaf_nimber);
                    if leaf_nimber == nimber {return true}
                }
                None => continue,
            }
        }

        return false;
        /*
        for leaf_index in 0..leaf_count
        {
            let mut modifier = 0;

            let entry = self.get(index); //borrow
            let part_count = entry.get_leaf_indices()[leaf_index].len();
            if part_count == 0 {
                self.remove_nimber(index, 0);
                if nimber == 0 {
                    return true;
                } else {
                    continue;
                }
            }
            //accumulate all the nimbers of the first (n-1) parts
            for part_index in 0..(part_count - 1) {
                let entry = self.get(index); //reborrow

                let part = entry.get_leaf_indices()[leaf_index][part_index];

                modifier ^= self.get_nimber_of_index_bounded(part, u16::MAX).unwrap();
            }
            //index of the last part of the current child game
            let entry = self.get(index); //reborrow
            let last_part = entry.get_leaf_indices()[leaf_index]
                .last()
                .unwrap();
            //if the last part has the _nimber == nimber xor modifier
            if let Some(_nimber) = self.get_nimber_of_index_bounded(*last_part, nimber ^ modifier) {
                self.remove_nimber(index, _nimber ^ modifier);
                if nimber == _nimber ^ modifier {
                    return true;
                }
                else{
                    continue;
                }
            }
        }
        //no child was found that has the nimber
        return false;
        */
    }
    fn set_nimber(&mut self, index: usize, nimber: u16) {
        self.get_mut(index).possible_nimbers = vec![nimber];
    }
    fn remove_nimber(&mut self, index: usize, nimber: u16) {
        let entry = self.get_mut(index);
        if entry.possible_nimbers[0] > nimber {
            return;
        }
        match entry.possible_nimbers.binary_search(&nimber) {
            Ok(i) => _ = entry.possible_nimbers.remove(i),
            Err(_) => (),
        }
    }

    fn generate_leaf_entries(&mut self, index: usize) {
        let entry = self.get_mut(index);
        if entry.leaf_indices.is_some() {
            return;
        }
        let mut child_games = entry.get_child_games();
        //sort by child_count, node_count
        child_games.sort_by(|a, b| a.get_max_nimber().cmp(&b.get_max_nimber()));
        child_games.sort_by(|a, b| a.get_parts().len().cmp(&b.get_parts().len()));

        let child_indices: Vec<Vec<usize>> = child_games
            .into_iter()
            .map(|child| self.create_indecies_of(&child))
            .collect();
        let entry = &mut self.data[index]; //reborrow
        entry.set_child_indices(child_indices);
    }

    //------------------Getters and setter---------------------------------//

    pub fn create_indecies_of(&mut self, g: &Whole) -> Vec<usize> {
        g.get_parts()
            .iter()
            .map(|part| self.create_index_of(part))
            .collect()
    }

    pub fn create_index_of(&mut self, g: &Part) -> usize {
        if let Some(index) = self.index_of(g) {
            return index;
        } else {
            return self.add_game(g.clone());
        }
    }
    pub fn index_of(&self, g: &Part) -> Option<usize> {
        return self.indecies.get(g).copied();
    }
    fn get_mut(&mut self, index: usize) -> &mut Entry<Part, Whole> {
        return &mut self.data[index];
    }
    fn get(&self, index: usize) -> &Entry<Part, Whole> {
        return &self.data[index];
    }
    pub fn add_game(&mut self, game: Part) -> usize {
        let entry = Entry::new(game.clone());
        let index = self.data.len();
        self.indecies.insert(game, index);
        self.data.push(entry);
        return index;
    }
}
