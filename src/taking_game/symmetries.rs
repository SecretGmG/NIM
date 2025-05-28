use super::TakingGame;
use std::collections::HashMap;
//Implements the symmetry finder for GeneralizedNimGame
impl TakingGame {
    ///Tries to find a symmetry by running a recursive algorithm
    pub fn find_symmetry(&self) -> Option<Vec<usize>> {
        //If there isn't an even amount of nodes it's impossible for every node to have a symmetry
        if self.node_count % 2 != 0 {
            return None;
        }
        if 0 != (0..self.get_node_count())
            .into_iter()
            .map(|node| {
                self.set_indices[node]
                    .iter()
                    .map(|set_index| self.sets_of_nodes[*set_index].len())
                    .fold(0, |a, b| a << 1 ^ b)
            })
            .fold(0, |a, b| a ^ b)
        {
            return None;
        }

        let sets_of_candidates = self.get_sets_of_candidates();

        if sets_of_candidates.iter().any(|v| (v.len() % 2) != 0) {
            return None;
        }

        let mut symmetries = vec![None; self.get_node_count()];

        return self.leads_to_contradiction(&mut symmetries, &sets_of_candidates);
    }

    /// sorts the nodes into sets that each have the same neighbour pattern
    fn get_sets_of_candidates(&self) -> Vec<Vec<usize>> {
        //key: sorted amount of
        //value: vec of nodes that have neighbours with exactly this amount of neighbours
        let mut map: HashMap<Vec<usize>, Vec<usize>> = HashMap::with_capacity(self.node_count);

        for node in 0..self.node_count {
            //generate list of neighbours_lengths

            let set_pattern: Vec<usize> = self.set_indices[node]
                .iter()
                .map(|set_index| self.sets_of_nodes[*set_index].len())
                .collect();

            //push nodes that have the same neighbour pattern
            match map.get_mut(&set_pattern) {
                Some(v) => v.push(node),
                None => _ = map.insert(set_pattern, vec![node]),
            }
        }
        return map.into_values().collect();
    }

    ///gets nodes that might be symmetric to a given node
    fn get_candidates(
        &self,
        node: usize,
        symmetries: &Vec<Option<usize>>,
        sets_of_candidates: &Vec<Vec<usize>>,
    ) -> Vec<usize> {
        let candidates = sets_of_candidates
            .iter()
            .find(|set| set.contains(&node))
            .expect("at least one set of candidates should contain the node");

        //filter for nodes that are not
        // * the node itself
        // * nodes already in a symmetry
        // * nodes that are in the same set
        return candidates
            .iter()
            .filter(|&candidate| self.can_be_symmetric(node, *candidate, symmetries))
            .copied()
            .collect();
    }

    fn can_be_symmetric(
        &self,
        node: usize,
        candidate: usize,
        symmetries: &Vec<Option<usize>>,
    ) -> bool {
        if node == candidate {
            return false;
        }
        if symmetries[candidate].is_some() {
            return false;
        }
        if self.set_indices[node]
            .iter()
            .any(|set_index| self.sets_of_nodes[*set_index].contains(&candidate))
        {
            return false;
        }
        if !self
            .get_neighbours(node)
            .filter_map(|n| symmetries[*n])
            .all(|n1| self.get_neighbours(candidate).any(|n2| n1 == *n2))
        {
            return false;
        }

        return true;
    }
    fn get_neighbours(&self, node: usize) -> impl Iterator<Item = &usize> {
        self.set_indices[node]
            .iter()
            .flat_map(|set_index| &self.sets_of_nodes[*set_index])
    }

    ///gets the first node in no symmetry
    fn get_next_free_node(&self, symmetries: &Vec<Option<usize>>) -> Option<usize> {
        for i in 0..self.get_node_count() {
            if symmetries[i].is_none() {
                return Some(i);
            }
        }
        return None;
    }

    /// ### Takes a vec of symmetries and checks if they do not lead to a contradiction by:
    ///
    ///   * Checking if there exists a root_node wich has a symmetry to another node that does not lead to a contradiction,
    ///     assuming the vec of previously definde symmetries.
    ///
    ///   * Every Iteration a new node is chosen and candidates of wich at least one HAS to be symmetric to the node are generated
    ///
    ///   * If any of these candidates is found to be symmetric to the root_node without contradiction the assumed symmetries must be valid
    ///
    ///   * This can be checked by adding the symmetry (root_node <==> candidate_node) to the vec of symmetries and recursively calling the function.
    ///
    ///
    fn leads_to_contradiction(
        &self,
        symmetries: &mut Vec<Option<usize>>,
        sets_of_candidates: &Vec<Vec<usize>>,
    ) -> Option<Vec<usize>> {
        //If there are none, return
        let next_node = match self.get_next_free_node(symmetries) {
            None => return Self::flatten_vec(symmetries),
            Some(node) => node,
        };

        //get nodes that might be symmetric to the root_node
        let candidates = self.get_candidates(next_node, symmetries, sets_of_candidates);

        //no symmetry candidates for the node means the start symmetry leads to a contradiction
        if candidates.len() == 0 {
            return None;
        }

        for candidate in candidates {
            symmetries[next_node] = Some(candidate);
            symmetries[candidate] = Some(next_node);

            match self.leads_to_contradiction(symmetries, sets_of_candidates) {
                Some(result) => return Some(result),
                None => {
                    symmetries[next_node] = None;
                    symmetries[candidate] = None;
                    continue;
                }
            }
        }
        return None;
    }
    ///flattenes a vec<Option<usize>> to a Option<Vec<usize>>
    fn flatten_vec(symmetries: &Vec<Option<usize>>) -> Option<Vec<usize>> {
        let mut result = vec![];
        for symmetry in symmetries {
            match symmetry {
                Some(symmetry) => result.push(*symmetry),
                None => return None,
            }
        }
        return Some(result);
    }
}

#[cfg(test)]
mod tests {

    use crate::taking_game::constructor::Constructor;

    #[test]
    fn test_hypercube_2_2() {
        let g = Constructor::hyper_cube(2, 2).build();
        assert!(g.find_symmetry().is_some());
    }
    #[test]
    fn test_hypercube_4_4() {
        let g = Constructor::hyper_cube(4, 4).build();
        assert!(g.find_symmetry().is_some());
    }
    #[test]
    fn test_hypercube_2_32() {
        let g = Constructor::hyper_cube(2, 32).build();
        assert!(g.find_symmetry().is_some());
    }
    #[test]
    fn test_hypercube_3_3() {
        let g = Constructor::hyper_cube(3, 3).build();
        assert!(g.find_symmetry().is_none());
    }
    #[test]
    fn test_hypertetrahedron_16() {
        let g = Constructor::hyper_tetrahedron(15).build();
        assert!(g.find_symmetry().is_none());
    }
}
