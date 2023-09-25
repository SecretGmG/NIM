use super::{TakingGame, util};
use std::collections::HashMap;
use super::util::compare_sorted;

impl TakingGame {
    #[allow(dead_code)]
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> TakingGame {
        return TakingGame {
            sets_of_nodes: Vec::new(),
            set_indices: Vec::new(),
            node_count: 0,
        };
    }
    ///creates and simplifies a GeneralisedTakingGame from a vec<vec<usize>>
    pub fn new(mut sets_of_nodes: Vec<Vec<usize>>) -> TakingGame {
        let nodes = Self::flatten_and_get_node_count(&mut sets_of_nodes);
        Self::remove_redundant_sets(&mut sets_of_nodes);
        let set_indices = Self::generate_set_indices(&sets_of_nodes, nodes);
        
        let mut taking_game =  TakingGame {
            sets_of_nodes,
            set_indices,
            node_count: nodes,
        };
        taking_game.sort();
        return taking_game;
    }
    ///flattens the indecies and then returns the nr of nodes
    fn flatten_and_get_node_count(sets_of_nodes: &mut Vec<Vec<usize>>) -> usize {
        let mut indices: Vec<usize> = sets_of_nodes.iter().flatten().copied().collect();
        indices.sort_unstable();
        indices.dedup();
        let mut map = HashMap::new();
        for i in 0..indices.len() {
            map.insert(indices[i], i );
        }
        {
            for i in 0..sets_of_nodes.len() {
                for j in 0..sets_of_nodes[i].len() {
                    sets_of_nodes[i][j] = *map.get(&sets_of_nodes[i][j]).expect("the indices were generated correctly");
                }
            }
        };
        return indices.len() ;
    }
    ///removes sets that are totally contained in other sets
    fn remove_redundant_sets(sets_of_nodes: &mut Vec<Vec<usize>>) {
        for i in 0..sets_of_nodes.len() {
            sets_of_nodes[i].sort_unstable();
            sets_of_nodes[i].dedup();
        }
        sets_of_nodes.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut i = 0;
        'outer: while i + 1 < sets_of_nodes.len() {
            if sets_of_nodes[i].len() == 0 {
                sets_of_nodes.remove(i);
                continue;
            }

            for bigger_set in &sets_of_nodes[(i + 1)..] {
                if util::is_subset(&sets_of_nodes[i], bigger_set) {
                    sets_of_nodes.remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
    }
    pub fn sort(&mut self) {
        loop {
            self.sort_sets_of_nodes_by_indices();
            let permutation = self.generate_index_remaping();
            if {
                (0..permutation.len())
                    .zip(&permutation)
                    .all(|(a, b)| a  == *b)
            } {
                return;
            }
            for i in 0..self.sets_of_nodes.len() {
                for j in 0..self.sets_of_nodes[i].len() {
                    self.sets_of_nodes[i][j] = permutation[self.sets_of_nodes[i][j]];
                }
            }
        }
    }
    fn generate_index_remaping(&self) -> Vec<usize> {
        let mut inverse_maping: Vec<usize> = (0..self.get_node_count()).collect();
        inverse_maping.sort_by(|a, b| util::node_comparer(*a, *b, &self.set_indices));
        let maping = util::inverse_permutation(inverse_maping);
        return maping;
    }

    ///sorts each set of nodes and sorts the sets of nodes
    fn sort_sets_of_nodes_by_indices(&mut self) {
        self.sets_of_nodes.iter_mut().for_each(|set_of_nodes| set_of_nodes.sort_unstable());
        self.sets_of_nodes.sort_by(compare_sorted);
        self.set_indices = Self::generate_set_indices(&self.sets_of_nodes, self.get_node_count());
    }
    ///gets a vec with each index storing all the set that contain the node with that index
    pub fn generate_set_indices(sets_of_nodes: &Vec<Vec<usize>>, nodes: usize) -> Vec<Vec<usize>> {
        let mut set_indices = vec![vec![]; nodes ];

        for i in 0..sets_of_nodes.len() {
            for node in &sets_of_nodes[i] {
                set_indices[*node ].push(i );
            }
        }
        for i in 0..nodes {
            set_indices[i ].sort_unstable();
        }
        return set_indices;
    }
}