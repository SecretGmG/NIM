use crate::vec_ops;

use super::generalized::{closed_generalized::ClosedGeneralizedNimGame, GeneralizedNimGame};
use std::collections::HashMap;
mod entry;
use entry::Entry;

pub struct Evaluator {
    data: Vec<Entry>,
    indecies: HashMap<ClosedGeneralizedNimGame, usize>,
}
impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            data: vec![],
            indecies: HashMap::new(),
        }
    }

    pub fn calc_nimber(&mut self, g: &GeneralizedNimGame) -> u16 {
        let parts_indices = self.create_indecies_of(&g);
        let mut nimber = 0;
        for part_index in parts_indices {
            nimber ^= self.get_nimber(part_index);
        }
        return nimber;
    }

    fn get_nimber(&mut self, index: usize) -> u16 {
        self.generate_child_entries(index);
        let nimber = loop {
            let entry = self.get_mut(index); //reborrow
            if entry.possible_nimbers.len() == 0 {
                unreachable!();
            }
            let possible_nimber = entry.possible_nimbers[0];
            if entry.possible_nimbers.len() == 1 {
                break possible_nimber;
            }
            if self.is_nimber(index, possible_nimber) {
                break possible_nimber;
            }
        };
        return nimber;
    }
    ///changes the possible nibers!
    fn is_nimber(&mut self, index: usize, nimber: u16) -> bool {
        let entry = self.get(index);
        if entry.possible_nimbers.binary_search(&nimber).is_err() {
            return false;
        }

        //test if there is a child game with every child_nimber < nimber
        loop{
            let entry = self.get(index); //reborrow
            let nimber_to_test = *entry.possible_nimbers.first().unwrap();

            //if the smallest nimber in possible nimber is > the nimber
            if nimber_to_test > nimber{
                return false;
            }
            if self.has_child_with_nimber(index, nimber_to_test){
                self.remove_nimber(index, nimber_to_test);
            }
            else{
                self.set_nimber(index, nimber_to_test);
                return nimber_to_test == nimber;
            }
        };
    }
    fn set_nimber(&mut self, index: usize, nimber: u16){
        self.get_mut(index).possible_nimbers = vec![nimber];
    }
    fn remove_nimber(&mut self, index: usize, nimber: u16){
        let entry = self.get_mut(index);
        match entry.possible_nimbers.binary_search(&nimber) {
            Ok(i) => _ = entry.possible_nimbers.remove(i),
            Err(_) => (),
        }
    }
    ///does not change the possible nimbers!
    fn has_child_with_nimber(&mut self, index: usize, nimber: u16) -> bool {
        self.generate_child_entries(index);

        let child_count = self.get(index).child_games.as_ref().unwrap().len();
        //if the node has no childs return zero;
        if child_count == 0 {
            return nimber != 0;
        }
        for child_index in 0..child_count {
            let mut modifier = nimber;

            let entry = self.get(index); //borrow
            let part_count = entry.child_games.as_ref().unwrap()[child_index].len();
            if part_count == 0 {
                if nimber == 0 {
                    return true;
                } else {
                    continue;
                }
            }
            //accumulate all the nimbers of the first (n-1) parts
            for part_index in 0..(part_count - 1) {
                let entry = self.get(index); //reborrow
                let part = entry.child_games.as_ref().unwrap()[child_index][part_index];
                modifier ^= self.get_nimber(part);
            }
            //index of the last part of the current child game
            let entry = self.get(index); //reborrow
            let last = entry.child_games.as_ref().unwrap()[child_index]
                .last()
                .unwrap();
            //if the last part is the modifier there is a child with the nimber
            if self.is_nimber(*last, modifier) {
                return true;
            }
        }
        //no child was found that has the nimber
        return false;
    }

    fn generate_child_entries(&mut self, index: usize) {
        let entry = self.get_mut(index);
        if entry.child_games.is_some() {
            return;
        }
        let child_games = entry.get_child_games();

        let mut child_indices: Vec<Vec<usize>> = child_games
            .into_iter()
            .map(|child| self.create_indecies_of(&child))
            .collect();
        let entry = &mut self.data[index]; //reborrow
                                           //sort by child_count, node_count
        vec_ops::sort_vec_of_vecs(&mut child_indices);
        entry.child_games = Some(child_indices);
    }

    //------------------Getters and setter---------------------------------//

    pub fn create_indecies_of(&mut self, g: &GeneralizedNimGame) -> Vec<usize> {
        g.get_parts()
            .iter()
            .map(|part| self.create_index_of(part))
            .collect()
    }

    pub fn create_index_of(&mut self, g: &ClosedGeneralizedNimGame) -> usize {
        if let Some(index) = self.index_of(g) {
            return index;
        } else {
            return self.add_game(g.clone());
        }
    }
    pub fn index_of(&self, g: &ClosedGeneralizedNimGame) -> Option<usize> {
        return self.indecies.get(g).copied();
    }
    fn get_mut(&mut self, index: usize) -> &mut Entry {
        return &mut self.data[index];
    }
    fn get(&self, index: usize) -> &Entry {
        return &self.data[index];
    }
    pub fn add_game(&mut self, game: ClosedGeneralizedNimGame) -> usize {
        let entry = Entry::new(game.clone());
        let index = self.data.len();
        self.indecies.insert(game, index);
        self.data.push(entry);
        return index;
    }
}
