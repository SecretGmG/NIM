use super::GeneralizedNimGame;


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
    pub fn get_easy_nimber(&self) -> Option<u16>{
        if self.groups.len() == 0 {
            return Some(0);
        }
        if self.groups.len() == 1 {
            return Some(self.nodes as u16);
        }
        return None;
    }
    #[allow(dead_code)]
    pub fn into_generalized(self) -> GeneralizedNimGame{
        return GeneralizedNimGame{parts: vec![self]}
    }
    pub fn get_groups(&self) -> &Vec<Vec<u16>>{&self.groups}
    pub fn get_group_indecies(&self) -> &Vec<Vec<u16>>{&self.group_indecies}
    pub fn get_node_count(&self) -> u16{self.nodes}
}

