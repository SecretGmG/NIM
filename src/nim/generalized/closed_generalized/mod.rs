use crate::nim::vec_ops;

use super::{data_base::DataBase, GeneralizedNimGame};

pub mod child_games;
pub mod impls;
pub mod new;
pub mod symmetries;

pub struct ClosedGeneralizedNimGame {
    groups: Vec<Vec<u16>>,
    /// neighbours[i] stores all groups containing i
    group_indecies: Vec<Vec<u16>>,
    ///the amount of nodes in groups <=> the biggest index
    nodes: u16,
}

//Constructors and basic functions for the GeneralizedNimGame
impl ClosedGeneralizedNimGame {
    /*
    pub fn is_symmetric(&self ) -> bool {
        match self.find_symmetry() {
            Some(_) => true,
            None => false,
        }
    }
    */

    pub fn get_easy_nimber(&self) -> Option<u16>{
        if self.groups.len() == 0 {
            return Some(0);
        }
        if self.groups.len() == 1 {
            return Some(self.nodes as u16);
        }
        return None;
    }

    pub fn get_nimber(&self, prev_seen: &mut DataBase) -> u16 {
        
        if let Some(nimber) = self.get_easy_nimber(){
            return nimber;
        }
        if let Some(nimber) = prev_seen.get(self) {
            return nimber;
        }

        return self.calculate_nimber(prev_seen);
    }
    pub fn into_generalized(self) -> GeneralizedNimGame{
        return GeneralizedNimGame{parts: vec![self]}
    }
    pub fn get_groups(&self) -> &Vec<Vec<u16>>{&self.groups}
    pub fn get_group_indecies(&self) -> &Vec<Vec<u16>>{&self.group_indecies}
    pub fn get_node_count(&self) -> u16{self.nodes}

    fn calculate_nimber(&self, prev_seen: &mut DataBase) -> u16 {
        let unique_child_games = self.get_unique_child_games();
        let nimbers: &mut Vec<u16> = &mut unique_child_games.into_iter().map(|g|g.get_nimber(prev_seen)).collect();
        let nimber = vec_ops::mex(nimbers);

        prev_seen.set(self, nimber);

        return nimber;
    }
}

