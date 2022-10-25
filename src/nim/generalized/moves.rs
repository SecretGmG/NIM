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

            let (lone_nodes, other_nodes) = self.collect_lone_nodes_and_other_nodes(group);
            self.append_moves_of_group(lone_nodes, other_nodes, &mut child_games);
        }

        child_games.sort_by(Self::cmp);
        child_games.dedup();

        return child_games;
    }

    fn append_moves_of_group(&self, lone_nodes: Vec<u16>, other_nodes: Vec<u16>, child_games: &mut Vec<GeneralizedNimGame>) {
        
        if other_nodes.len() > 128 {panic!("This game is too complex!")}
        
        for lone_nodes_to_remove in 0..(lone_nodes.len()+1)
        {
            let mask_bound = 1<<other_nodes.len();
            let start_value = if lone_nodes_to_remove == 0 {1} else {0};
            for mask in start_value..mask_bound
            {
                child_games.push(
                    self.get_child(
                        &lone_nodes, 
                        &other_nodes,
                        lone_nodes_to_remove as u16,
                        mask
                        )
                    );
            }
        }
    }
    fn collect_lone_nodes_and_other_nodes(&self, group :&Vec<u16>) -> (Vec<u16>, Vec<u16>){
        let mut lone_nodes_in_group = vec![];
        let mut other_nodes_in_group = vec![];

        for node in group{
            let node = *node;
            if self.neighbours[node as usize].len() == 0{
                lone_nodes_in_group.push(node);
            }
            else{
                other_nodes_in_group.push(node);
            }
        }
        return (lone_nodes_in_group, other_nodes_in_group)
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

        let mut i = 0;
        let mut j = 0;
        while i<neighbours1.len() && j<neighbours2.len() {
            if neighbours1[i] == node2 {i+=1;continue;}
            if neighbours2[j] == node1 {j+=1;continue;}

            if neighbours1[i] != neighbours2[j] {return false;}
            i+=1;
            j+=1;
        }


        return true;
    }

    fn get_child(&self, lone_nodes :&Vec<u16>, other_nodes :&Vec<u16>, lone_nodes_to_remove: u16, mask :u128) -> GeneralizedNimGame{
        let mut nodes_to_remove = vec![];
        for i in 0..lone_nodes_to_remove{
            nodes_to_remove.push(lone_nodes[i as usize]);
        }
        let mut mask = mask;
        for i in 0..other_nodes.len(){
            if mask %2 == 1{
                nodes_to_remove.push(other_nodes[i]);
            }
            mask >>= 1;
        }
        let child = self.remove_nodes(&mut nodes_to_remove).unwrap();
        return child;
    }
    ///removes all nodes specified in the argument
    pub fn remove_nodes(&self, nodes_to_remove: &mut Vec<u16>) -> Option<GeneralizedNimGame>{

        nodes_to_remove.sort();

        if *nodes_to_remove.last().unwrap_or(&u16::MAX) > self.nodes 
        {
            return None;
        }
        
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