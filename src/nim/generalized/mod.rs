pub mod display;
pub mod symmetries;
pub mod moves;
pub mod nimber;
pub mod impls;
pub mod data_base;
pub mod new;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game
#[derive(Eq)]
pub struct GeneralizedNimGame{
    groups :Vec<Vec<u16>>,
    /// neighbours[i] stores all nodes neighbouring i in ascending order (deduped)
    neighbours :Vec<Vec<u16>>,
    ///the amount of nodes in groups <=> the biggest index
    nodes :u16
}

//Constructors and basic functions for the GeneralizedNimGame
impl GeneralizedNimGame{
    /// gets all neighbours of a given node
    pub fn get_neighbours(&self, node :u16) -> &Vec<u16>{
        return &self.neighbours[node as usize];
    }
}