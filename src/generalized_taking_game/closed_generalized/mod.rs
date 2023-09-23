use super::TakingGame;

pub mod moves;
pub mod new;
pub mod symmetries;

#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ClosedTakingGamePart {
    sets_of_nodes: Vec<Vec<u16>>,
    set_indices: Vec<Vec<u16>>,
    nodes: u16,
}

//Constructors and basic functions for the GeneralizedNimGame
impl ClosedTakingGamePart {
    pub fn get_possible_nimbers(&self) -> Vec<u16>{
        if self.find_symmetry().is_some() {
            return vec![0];
        }else{
            return (0..=self.get_node_count()).collect();
        }
    }
    pub fn into_generalized(self) -> TakingGame {
        return TakingGame { parts: vec![self] };
    }
    pub fn get_sets_of_nodes(&self) -> &Vec<Vec<u16>> {
        &self.sets_of_nodes
    }
    pub fn get_node_count(&self) -> u16 {
        self.nodes
    }
}
