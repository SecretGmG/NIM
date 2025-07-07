pub mod constructor;
mod new;
mod impls;
mod moves;
mod symmetries;
pub mod util;

/// A generalized representation of an impartial "taking game".
/// 
/// This struct implements tools to efficiently compute the nimber 
/// (Grundy number) for complex taking games by modeling them as 
/// collections of node sets.
#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TakingGame {
    sets_of_nodes: Vec<Vec<usize>>,
    set_indices: Vec<Vec<usize>>,
    node_count: usize,
}
impl TakingGame {
    pub fn get_sets_of_nodes(&self) -> &Vec<Vec<usize>> {
        &self.sets_of_nodes
    }
    pub fn get_node_count(&self) -> usize {
        self.node_count
    }
    pub fn get_set_indices(&self) -> &Vec<Vec<usize>> {
        &self.set_indices
    }
}