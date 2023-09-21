mod flatten;
mod sort;


use super::ClosedGeneralizedNimGame;
use flatten::flatten;
use sort::sort;
use sort::get_group_indecies;

impl ClosedGeneralizedNimGame {
    #[allow(dead_code)]
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> ClosedGeneralizedNimGame {
        return ClosedGeneralizedNimGame {
            sets_of_nodes: Vec::new(),
            set_indices: Vec::new(),
            nodes: 0,
        };
    }
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(groups: Vec<Vec<u16>>) -> ClosedGeneralizedNimGame {
        let mut groups = groups; //make mutable

        let nodes = flatten(&mut groups);
        sort(&mut groups, nodes);
        let group_indecies = get_group_indecies(&groups, nodes);

        return ClosedGeneralizedNimGame {
            sets_of_nodes: groups,
            set_indices: group_indecies,
            nodes: nodes,
        };
    }
}
