use super::GeneralizedNimGame;

//implements the generation of moves;
impl GeneralizedNimGame{
    pub fn get_unique_child_games(&self) -> Vec<GeneralizedNimGame> 
    {
        let mut child_games = vec![];

        let mut processed_groups = vec![];


        for group in &self.groups
        {

            //if self.group_is_symmetric_to_any(group, &processed_groups){continue;}


            processed_groups.push(group);

            let mut lone_nodes_in_group = vec![];
            let mut other_nodes_in_group = vec![];

            //collecting the lone nodes and other nodes
            for node in group{
                let node = *node;
                if self.neighbours[node as usize].len() == 0{
                    lone_nodes_in_group.push(node);
                }
                else{
                    other_nodes_in_group.push(node);
                }
            }

            let lone_nodes_to_remove_bound = lone_nodes_in_group.len()+1;
            for number_of_lone_nodes_to_remove in 0..lone_nodes_to_remove_bound
            {
                if other_nodes_in_group.len() > 128 {panic!("This game is too complex!")}
                let mask_bound = 1<<other_nodes_in_group.len();
                for mask in 1..mask_bound
                {
                    child_games.push(
                        self.get_child(
                            &lone_nodes_in_group, 
                            &other_nodes_in_group,
                            number_of_lone_nodes_to_remove as u16,
                            mask
                            )
                        );
                }
            }
        }

        child_games.sort_by(Self::cmp);
        child_games.dedup();
        return child_games;
    }

    fn group_is_symmetric_to_any(&self, group :&Vec<u16>, processed_groups :&Vec<&Vec<u16>>) -> bool{
        for processed in processed_groups{
            if self.group_is_symmetric(group, processed){
                return true;
            }
        }
        return false;
    }
    fn group_is_symmetric(&self, group :&Vec<u16>, other :&Vec<u16>) -> bool{
        if group.len() != other.len() {return false;}

        for i in 0..group.len(){

            let node1 = group[i];
            let node2 = other[i];

            let neighbours1 = &self.neighbours[node1 as usize];
            let neighbours2 = &self.neighbours[node2 as usize];

            if !Self::have_the_same_neighbours(node1, node2, neighbours1, neighbours2) {return false;}
            
        }
        return true;
    }
    fn have_the_same_neighbours(node1 :u16, node2 :u16, neighbours1 :&Vec<u16>, neighbours2 :&Vec<u16>) -> bool{
        if neighbours1.len() != neighbours2.len() {return false;}

        for i in 0..neighbours1.len(){
            if neighbours1[i] == neighbours2[i] ||
                (
                    neighbours1[i] == node2 &&
                    neighbours2[i] == node1
                )            
            {}
            else{
                return false;
            }


        }


        return true;
    }

    fn get_child(&self, lone_nodes_in_group :&Vec<u16>, other_nodes_in_group :&Vec<u16>, nr_of_lone_nodes_to_remove: u16, mask :u128) -> GeneralizedNimGame{
        let mut nodes_to_remove = vec![];
        for i in 0..nr_of_lone_nodes_to_remove{
            nodes_to_remove.push(lone_nodes_in_group[i as usize]);
        }

        let mut mask = mask;

        for i in 0..other_nodes_in_group.len(){
            if mask %2 == 1{
                nodes_to_remove.push(other_nodes_in_group[i]);
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
        
        let mut new_groups = vec![];

        for group in &self.groups{
            let mut new_group = vec![];
            for node in group{
                if nodes_to_remove.binary_search(&node).is_err(){ //if it is not in the nodes to remove
                    new_group.push(*node);
                }
            }
            new_groups.push(new_group);
        }

        let new_game = GeneralizedNimGame::new(new_groups); 

        return Some(new_game);
    }
    pub fn keep_nodes(&self, nodes_to_keep: &mut Vec<u16>) -> Option<GeneralizedNimGame>{

        nodes_to_keep.sort();

        if *nodes_to_keep.last().unwrap() > self.nodes {
            return None;
        }
        
        let mut new_groups = vec![];

        for group in &self.groups{
            let mut new_group = vec![];
            for node in group{
                if nodes_to_keep.binary_search(&node).is_ok(){ //if it is in the nodes to keep
                    new_group.push(*node);
                }
            }
            new_groups.push(new_group);
        }
        return Some(GeneralizedNimGame::new(new_groups))
    }
}