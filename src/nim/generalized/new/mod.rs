use super::GeneralizedNimGame;

mod flatten;
mod remove_unnecessary;
mod neighbours;
mod sort;



use flatten::flatten_and_get_node_count;
use remove_unnecessary::remove_unnecessary_data;
use neighbours::generate_all_neighbours;
use sort::sort;

impl GeneralizedNimGame {
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> GeneralizedNimGame {
        return GeneralizedNimGame {
            groups: Vec::new(),
            neighbours: Vec::new(),
            nodes: 0,
        };
    }
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(groups: Vec<Vec<u16>>) -> GeneralizedNimGame {
        let mut groups = groups; //make mutable

        let nodes = flatten_and_get_node_count(&mut groups);

        remove_unnecessary_data(&mut groups);

        let mut neighbours = generate_all_neighbours(&groups, nodes);

        sort(&mut groups, &mut neighbours, nodes);

        return GeneralizedNimGame {
            groups: groups,
            nodes: nodes,
            neighbours: neighbours,
        };
    }
}