use crate::nim::vec_ops;

use super::data_base::DataBase;

pub mod child_games;
pub mod impls;
pub mod new;
pub mod symmetries;

pub struct ClosedGeneralizedNimGame {
    groups: Vec<Vec<u16>>,
    /// neighbours[i] stores all nodes neighbouring i in ascending order (deduped)
    neighbours: Vec<Vec<u16>>,
    ///the amount of nodes in groups <=> the biggest index
    nodes: u16,
}

//Constructors and basic functions for the GeneralizedNimGame
impl ClosedGeneralizedNimGame {
    /// gets all neighbours of a given node
    pub fn get_neighbours(&self, node: u16) -> &Vec<u16> {
        return &self.neighbours[node as usize];
    }
    pub fn is_symmetric(&self ) -> bool {
        match self.find_symmetry() {
            Some(_) => true,
            None => false,
        }
    }
    pub fn get_nimber(&self, prev_seen: &mut DataBase) -> u16 {
        if self.groups.len() == 0 {
            return 0;
        }
        if self.groups.len() == 1 {
            return self.nodes as u16;
        }

        if let Some(nimber) = prev_seen.get(self) {
            return nimber;
        }

        return self.calculate_nimber(prev_seen);
    }

    fn calculate_nimber(&self, prev_seen: &mut DataBase) -> u16 {
        let unique_child_games = self.get_unique_child_games();
        let nimbers: &mut Vec<u16> = &mut unique_child_games.into_iter().map(|g|g.get_nimber(prev_seen)).collect();
        let nimber = vec_ops::mex(nimbers);

        prev_seen.set(self, nimber);

        return nimber;
    }
}

