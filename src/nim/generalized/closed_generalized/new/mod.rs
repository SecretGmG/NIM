mod flatten;
mod neighbours;
mod remove_unnecessary;
mod sort;

use super::ClosedGeneralizedNimGame;
use flatten::flatten_and_get_node_count;
use neighbours::generate_all_neighbours;
use remove_unnecessary::remove_unnecessary_data;
use sort::sort;

impl ClosedGeneralizedNimGame {
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> ClosedGeneralizedNimGame {
        return ClosedGeneralizedNimGame {
            groups: Vec::new(),
            neighbours: Vec::new(),
            nodes: 0,
        };
    }
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(groups: Vec<Vec<u16>>) -> ClosedGeneralizedNimGame {
        let mut groups = groups; //make mutable

        let nodes = flatten_and_get_node_count(&mut groups);

        remove_unnecessary_data(&mut groups);

        let mut neighbours = generate_all_neighbours(&groups, nodes);

        sort(&mut groups, &mut neighbours, nodes);

        return ClosedGeneralizedNimGame {
            groups: groups,
            nodes: nodes,
            neighbours: neighbours,
        };
    }
}
