use std::collections::HashMap;

use super::GeneralizedNimGame;

//Implements the symmetry finder for GeneralizedNimGame
impl GeneralizedNimGame{

    pub fn is_symmetric(&self) -> bool{
        match self.find_symmetry() {
            Some(_) => true,
            None => false,
        }
    }



    ///Tries to find a symmetry by running a recursive algorithm
    pub fn find_symmetry(&self) -> Option<Vec<u16>>{

        //If there isn't an even amount of nodes it's impossible for every node to have a symmetry
        if self.nodes%2 != 0 {return None;}

        let sets_of_candidates = self.get_sets_of_candidates();

        if Self::cannot_be_symmetric(&sets_of_candidates) {
            return None;
        }

        return self.leads_to_contradiction(&mut vec![None; self.nodes as usize], &sets_of_candidates);
    }


    ///checks if a set of sets candidates could be symmetric
    ///by checking if every set has an even amount of members 
    fn cannot_be_symmetric(sets_of_candidates :&Vec<Vec<u16>>) -> bool{
        return !sets_of_candidates.iter().all(|v| (v.len() % 2) == 0);
    }



    fn get_sets_of_candidates(&self) -> Vec<Vec<u16>>{
        
        //key: sorted amount of 
        //value: vec of nodes that have neighbours with exactly this amount of neighbours
        let mut map: HashMap<Vec<u16>, Vec<u16>> = HashMap::new();


        for node in 0..self.nodes {

            //generate list of neighbours_lengths

            let mut neighbours_lengths = vec![];

            for neighbour in self.get_neighbours(node){
                let amount_of_neighbours_neighbours = self.get_neighbours(*neighbour).len() as u16;

                match neighbours_lengths.binary_search(&amount_of_neighbours_neighbours) {
                    Ok(_) => {} // element already in vector 
                    Err(pos) => neighbours_lengths.insert(pos, amount_of_neighbours_neighbours),
                }
            }

            //push nodes that have the same neighbour pattern
            match map.get_mut(&neighbours_lengths){
                Some(v) => v.push(node),
                None => _ = map.insert(neighbours_lengths, vec![node]),
            }
        }
        return map.values().cloned().collect();
    }

    ///gets nodes that might be symmetric tho a given node
    fn get_candidates(&self, node :u16, symmetries :&Vec<Option<u16>>, sets_of_candidates :&Vec<Vec<u16>>) -> Vec<u16>{
        
        let mut candidates: &Vec<u16> = &vec![];

        //get the set of candidates in wich the node is
        for set_of_candidates in sets_of_candidates{
            if set_of_candidates.contains(&node){
                candidates = set_of_candidates;
                break;
            }
        }
        
        //generate a new vec without:
        // * the node itself
        // * nodes already in a symmetry
        // * nodes that are in the same group
        let mut final_candidates = vec![];

        for i in 0..candidates.len(){
            if candidates[i] == node || symmetries.contains(&Some(candidates[i])) || self.get_neighbours(node).contains(&candidates[i]) {continue;}
            final_candidates.push(candidates[i]);
        }


        return final_candidates.clone();
    }

    ///gets the first node in no symmetry
    fn get_next_node(&self, symmetries :&Vec<Option<u16>>) -> Option<u16>{
        for i in 0..self.nodes {
            if symmetries[i as usize] == Option::None {return Some(i);} 
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
    fn leads_to_contradiction(&self, symmetries :&mut Vec<Option<u16>>, sets_of_candidates :&Vec<Vec<u16>>) -> Option<Vec<u16>>{
        
        //If there are none, return
        let root_node = match self.get_next_node(symmetries){
            None => return Self::flatten_vec(symmetries),
            Some(node) => node
        };


        //get nodes that might be symmetric to the root_node
        let candidates = self.get_candidates(root_node, symmetries, sets_of_candidates);
        
        //no symmetry candidates for the node means the start symmetry leads to a contradiction
        if candidates.len() == 0 {
            return None
        }
        
        //a list of all nodes that must be a neighbour of the candidate for the candidate not to be a contradiction
        let mut neighbours_of_symmetry = Vec::new();
        
        for neighbour in self.get_neighbours(root_node){
            match symmetries[*neighbour as usize]{
                Some(symmetric_neighbour) => neighbours_of_symmetry.push(symmetric_neighbour),
                None => continue
            }
        }
        
        for candidate in candidates {
            
            //if it leads to a contradiction go to the next candidate otherwise return Some(symmetry)
            
            //if a node of neighbours Of Symmetry were not in candidatesNeighbours that would be a contradiction
            if !neighbours_of_symmetry.iter().all(|neighbour_of_symmetry| self.get_neighbours(candidate).contains(neighbour_of_symmetry)) {
                return None;
            }
            
            //add the new symmetry to the symmetries
            symmetries[root_node as usize] = Some(candidate);
            symmetries[candidate as usize] = Some(root_node);
            
            match self.leads_to_contradiction(symmetries, sets_of_candidates){
                //If there is no contradiction, return Some(result)
                Some(result) => {
                    return Some(result)
                }

                //If there is, remove the previously added symmetry and continue
                None => {
                    symmetries[root_node as usize] = None;
                    symmetries[candidate as usize] = None;
                    continue;
                }
            }
        } 
        return  None;
    }

    ///flattenes a vec<Option<u16>> to a Option<Vec<u16>>
    fn flatten_vec(symmetries: &Vec<Option<u16>>) ->Option<Vec<u16>> {
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