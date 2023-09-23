use std::collections::HashMap;

use super::ClosedTakingGamePart;

//Implements the symmetry finder for GeneralizedNimGame
impl ClosedTakingGamePart {
    ///Tries to find a symmetry by running a recursive algorithm
    pub fn find_symmetry(&self) -> Option<Vec<usize>> {
        //If there isn't an even amount of nodes it's impossible for every node to have a symmetry
        if self.nodes % 2 != 0 {
            return None;
        }

        let sets_of_candidates = self.get_sets_of_candidates();

        if sets_of_candidates.iter().all(|v| (v.len() % 2) == 0) {
            return None;
        }

        let mut symmetries = vec![None; self.get_node_count() ];

        return self.leads_to_contradiction(&mut symmetries, &sets_of_candidates);
    }

    /// sorts the nodes into sets that each have the same neighbour pattern
    fn get_sets_of_candidates(&self) -> Vec<Vec<usize>> {
        //key: sorted amount of
        //value: vec of nodes that have neighbours with exactly this amount of neighbours
        let mut map: HashMap<Vec<usize>, Vec<usize>> = HashMap::new();

        for node in 0..self.nodes {
            //generate list of neighbours_lengths
            let mut neighbours_lengths = vec![];

            for neighbour in &self.set_indices[node ] {
                let amount_of_neighbours_neighbours =
                    self.set_indices[*neighbour ].len() ;

                match neighbours_lengths.binary_search(&amount_of_neighbours_neighbours) {
                    Ok(_) => {} // element already in vector
                    Err(pos) => neighbours_lengths.insert(pos, amount_of_neighbours_neighbours),
                }
            }

            //push nodes that have the same neighbour pattern
            match map.get_mut(&neighbours_lengths) {
                Some(v) => v.push(node),
                None => _ = map.insert(neighbours_lengths, vec![node]),
            }
        }
        return map.into_values().collect();
    }

    ///gets nodes that might be symmetric tho a given node
    fn get_candidates(
        &self,
        node: usize,
        symmetries: &Vec<Option<usize>>,
        sets_of_candidates: &Vec<Vec<usize>>,
    ) -> Vec<usize> {
        let mut candidates: &Vec<usize> = &vec![];
        //get the set of candidates in wich the node is
        for sets_of_candidates in sets_of_candidates {
            if sets_of_candidates.contains(&node) {
                candidates = sets_of_candidates;
                break;
            }
        }

        //generate a new vec without:
        // * the node itself
        // * nodes already in a symmetry
        // * nodes that are in the same group
        let mut final_candidates = vec![];

        for i in 0..candidates.len() {
            if candidates[i] == node {
                continue;
            }
            if symmetries.binary_search(&Some(candidates[i])).is_ok() {
                continue;
            }
            if self.set_indices[node ]
                .binary_search(&candidates[i])
                .is_ok()
            {
                continue;
            }
            final_candidates.push(candidates[i]);
        }

        return final_candidates;
    }

    ///gets the first node in no symmetry
    fn get_next_node(&self, symmetries: &Vec<Option<usize>>) -> Option<usize> {
        for i in 0..self.nodes {
            if symmetries[i ] == Option::None {
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
        let root_node = match self.get_next_node(symmetries) {
            None => return Self::flatten_vec(symmetries),
            Some(node) => node,
        };

        //get nodes that might be symmetric to the root_node
        let candidates = self.get_candidates(root_node, symmetries, sets_of_candidates);

        //no symmetry candidates for the node means the start symmetry leads to a contradiction
        if candidates.len() == 0 {
            return None;
        }

        //a list of all nodes that must be a neighbour of the candidate for the candidate not to be a contradiction
        let mut neighbours_of_symmetry = Vec::new();
        //this seems weird!!!------------------------------------------------
        for neighbour in &self.set_indices[root_node ] {
            match symmetries[*neighbour ] {
                Some(symmetric_neighbour) => neighbours_of_symmetry.push(symmetric_neighbour),
                None => continue,
            }
        }

        for candidate in candidates {
            //if it leads to a contradiction go to the next candidate otherwise return Some(symmetry)

            //if a node of neighbours Of Symmetry were not in candidatesNeighbours that would be a contradiction
            if !neighbours_of_symmetry.iter().all(|neighbour_of_symmetry| {
                self.set_indices[candidate ].contains(neighbour_of_symmetry)
            }) {
                return None;
            }

            //add the new symmetry to the symmetries
            symmetries[root_node ] = Some(candidate);
            symmetries[candidate ] = Some(root_node);

            match self.leads_to_contradiction(symmetries, sets_of_candidates) {
                //If there is no contradiction, return Some(result)
                Some(result) => return Some(result),

                //If there is, remove the previously added symmetry and continue
                None => {
                    symmetries[root_node ] = None;
                    symmetries[candidate ] = None;
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
