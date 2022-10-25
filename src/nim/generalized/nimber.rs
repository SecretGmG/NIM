use super::GeneralizedNimGame;
use super::data_base::DataBase;





impl GeneralizedNimGame{

    pub fn calculate_nimber(&self, prev_seen :&mut DataBase) -> u16{

        let mut total_nimber = 0;    

        let parts = self.get_split();

        for part in parts{     
            
            let part_nimber = part.get_nimber_with_mex(prev_seen);

            total_nimber ^= part_nimber;
        }
        return total_nimber;
    }

    fn get_nimber_with_mex(&self, prev_seen :&mut DataBase) -> u16{

        if self.groups.len() == 0 {return 0;}
        if self.groups.len() == 1 {return self.nodes as u16}
        
        if let Some(nimber) = prev_seen.get(self){return nimber;}

        
        if self.is_symmetric() {
            prev_seen.set(self, 0);
            return 0;
        }
        


        let mut child_nimbers = vec![];

        let unique_child_games = self.get_unique_child_games();

        for child in unique_child_games {
            child_nimbers.push(child.calculate_nimber(prev_seen));
        }

        let nimber = Self::mex(&mut child_nimbers); 

        prev_seen.set(self, nimber);

        return nimber;
    }

    fn mex(nums: &mut Vec<u16>) -> u16{
        nums.sort();
        let mut i: u16 = 0;
        while nums.binary_search(&i).is_ok() {i+=1;}
        return i;
    }

    ///returns all independent parts of the GeneralizedNimGame
    pub fn get_split(&self) -> Vec<GeneralizedNimGame>{
        
        let mut processed_nodes = vec![];
        let mut parts = vec![];

        for i in 0..self.nodes
        {
            if !processed_nodes.contains(&i) //if the node is not already processed
            {
                let mut current_nodes = vec![];
                
                processed_nodes.push(i); //push i to the processed nodes
                current_nodes.push(i); //push i to the nodes to process
                
                for neighbour in &self.neighbours[i as usize]{
                    current_nodes.push(*neighbour); //push neighbours of i to the nodes to process
                }
                
                let mut new_part = vec![];

                while current_nodes.len() != 0 //while there are nodes to process
                {
                    let current_node = current_nodes.pop().unwrap();

                    new_part.push(current_node);

                    for neighbour in &self.neighbours[current_node as usize]
                    {
                        let neighbour = *neighbour;
                        if !processed_nodes.contains(&neighbour){ //if the node has not already been processed
                            
                            processed_nodes.push(neighbour);
                            current_nodes.push(neighbour);
                        }
                    }
                }
                if new_part.len() != 0{
                    parts.push(self.keep_nodes(&mut new_part).unwrap());
                }
            }
        }

        return parts;
    }
}