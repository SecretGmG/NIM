use super::GeneralizedNimGame;

//implements the generation of moves;
impl GeneralizedNimGame{
    pub fn get_unique_child_games(&self) -> Vec<GeneralizedNimGame> 
    {
        let mut childGames = vec![];

        for group in &self.neighbours
        {
            let mut loneNodesInGroup = vec![];
            let mut otherNodesInGroup = vec![];

            for node in group{
                let node = *node;
                if self.neighbours[node as usize].len() == 0{
                    loneNodesInGroup.push(node);
                }
                else{
                    otherNodesInGroup.push(node);
                }
            }

            for number_of_lone_nodes_to_remove in 0..loneNodesInGroup.len()
            {
                for mask in 1..(1 << group.len())
                {
                    childGames.push(
                        self.getChild(
                            &loneNodesInGroup, 
                            &otherNodesInGroup,
                            number_of_lone_nodes_to_remove as u16,
                            mask
                            )
                        );
                }
            }
        }
        return childGames;
    }

    fn getChild(&self, loneNodesInGroup :&Vec<u16>, nodesWithNeighboursInGroup :&Vec<u16>, numberOfLoneNodesToRemove: u16, mask :u128) -> GeneralizedNimGame{

        let mut nodesToRemove = vec![];
        for i in 0..numberOfLoneNodesToRemove{
            nodesToRemove.push(loneNodesInGroup[i as usize]);
        }

        let mut mask = mask;

        for i in 0..nodesWithNeighboursInGroup.len(){
            if mask %2 == 1{
                nodesToRemove.push(nodesWithNeighboursInGroup[i]);
            }
            mask >>= 1;
        }

        return self.remove_nodes(&mut nodesToRemove).unwrap();
    }

    pub fn remove_nodes(&self, _move: &mut Vec<u16>) -> Option<GeneralizedNimGame>{

        _move.sort();

        if *_move.last().unwrap() > self.nodes {return None;}
        
        let mut newGroups = vec![];

        for group in &self.groups{
            let mut newGroup = vec![];
            for node in group{
                if _move.binary_search(&node).is_err(){ //if it is not in the nodes to remove
                    newGroup.push(*node);
                }
            }
            newGroups.push(newGroup);
        }
        return Some(GeneralizedNimGame::new(newGroups))
    }
    pub fn keep_nodes(&self, _move: &mut Vec<u16>) -> Option<GeneralizedNimGame>{

        _move.sort();

        if *_move.last().unwrap() > self.nodes {return None;}
        
        let mut newGroups = vec![];

        for group in &self.groups{
            let mut newGroup = vec![];
            for node in group{
                if _move.binary_search(&node).is_ok(){ //if it is in the nodes to keep
                    newGroup.push(*node);
                }
            }
            newGroups.push(newGroup);
        }
        return Some(GeneralizedNimGame::new(newGroups))
    }
}