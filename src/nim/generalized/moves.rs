use super::GeneralizedNimGame;

//implements the generation of moves;
impl GeneralizedNimGame{
    pub fn get_unique_child_games(&self) -> Vec<GeneralizedNimGame> 
    {
        let mut childGames = vec![];

        for group in &self.groups
        {
            let mut loneNodesInGroup = vec![];
            let mut otherNodesInGroup = vec![];

            //collecting the lone nodes and other nodes
            for node in group{
                let node = *node;
                if self.neighbours[node as usize].len() == 0{
                    loneNodesInGroup.push(node);
                }
                else{
                    otherNodesInGroup.push(node);
                }
            }

            let lone_nodes_to_remove_bound = loneNodesInGroup.len()+1;
            for number_of_lone_nodes_to_remove in 0..lone_nodes_to_remove_bound
            {
                if otherNodesInGroup.len() > 128 {panic!("This game is too complex!")}
                let mask_bound = 1<<otherNodesInGroup.len();
                for mask in 1..mask_bound
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

        childGames.sort_by(Self::cmp);
        childGames.dedup();
        return childGames;
    }

    fn getChild(&self, loneNodesInGroup :&Vec<u16>, nodesWithNeighboursInGroup :&Vec<u16>, numberOfLoneNodesToRemove: u16, mask :u128) -> GeneralizedNimGame{
        let mut nodes_to_remove = vec![];
        for i in 0..numberOfLoneNodesToRemove{
            nodes_to_remove.push(loneNodesInGroup[i as usize]);
        }

        let mut mask = mask;

        for i in 0..nodesWithNeighboursInGroup.len(){
            if mask %2 == 1{
                nodes_to_remove.push(nodesWithNeighboursInGroup[i]);
            }
            mask >>= 1;
        }

        let child = self.remove_nodes(&mut nodes_to_remove).unwrap();
        return child;
    }
    ///removes all nodes specified in the argument
    pub fn remove_nodes(&self, nodes_to_remove: &mut Vec<u16>) -> Option<GeneralizedNimGame>{

        nodes_to_remove.sort();

        if *nodes_to_remove.last().unwrap() > self.nodes {return None;}
        
        let mut newGroups = vec![];

        for group in &self.groups{
            let mut newGroup = vec![];
            for node in group{
                if nodes_to_remove.binary_search(&node).is_err(){ //if it is not in the nodes to remove
                    newGroup.push(*node);
                }
            }
            newGroups.push(newGroup);
        }

        let newGame = GeneralizedNimGame::new(newGroups); 

        return Some(newGame);
    }
    pub fn keep_nodes(&self, nodes_to_keep: &mut Vec<u16>) -> Option<GeneralizedNimGame>{

        nodes_to_keep.sort();

        if *nodes_to_keep.last().unwrap() > self.nodes {
            return None;
        }
        
        let mut newGroups = vec![];

        for group in &self.groups{
            let mut newGroup = vec![];
            for node in group{
                if nodes_to_keep.binary_search(&node).is_ok(){ //if it is in the nodes to keep
                    newGroup.push(*node);
                }
            }
            newGroups.push(newGroup);
        }
        return Some(GeneralizedNimGame::new(newGroups))
    }
}