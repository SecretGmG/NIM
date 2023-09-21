use super::TakingGame;

pub mod moves;
pub mod new;
pub mod symmetries;

#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ClosedGeneralizedNimGame {
    sets_of_nodes: Vec<Vec<u16>>,
    set_indices: Vec<Vec<u16>>,
    nodes: u16,
}

//Constructors and basic functions for the GeneralizedNimGame
impl ClosedGeneralizedNimGame {
    pub fn get_easy_nimber(&self) -> Option<u16> {
        if self.sets_of_nodes.len() == 0 {
            return Some(0);
        }
        if self.sets_of_nodes.len() == 1 {
            return Some(self.nodes as u16);
        }
        return None;
    }
    #[allow(dead_code)]
    pub fn into_generalized(self) -> TakingGame {
        return TakingGame { parts: vec![self] };
    }
    pub fn get_sets_of_nodes(&self) -> &Vec<Vec<u16>> {
        &self.sets_of_nodes
    }
    pub fn get_set_indices(&self) -> &Vec<Vec<u16>>{
        &self.set_indices
    }
    pub fn get_groups(&self) -> &Vec<Vec<u16>> {
        &self.sets_of_nodes
    }
    pub fn get_node_count(&self) -> u16 {
        self.nodes
    }
}
