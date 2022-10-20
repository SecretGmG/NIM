use super::GeneralizedNimGame;


impl GeneralizedNimGame{

    pub fn calculate_nimber(&self) -> u16{
        if self.groups.len() == 0 {return 0;}
        if self.groups.len() == 1 {return self.groups[0].len() as u16}
        if self.is_symmetric() {return 0;}
        

        let mut total_nimber = 0;

        let parts = self.get_split();

        for part in parts{       
            total_nimber ^= Self::get_nimbers_with_mex(part);
        }
        
        return total_nimber;
    }
    fn get_nimbers_with_mex(generalizedNimGame : GeneralizedNimGame) -> u16{


        let mut childNimbers = vec![];

        for child in generalizedNimGame.get_unique_child_games() {
            childNimbers.push(child.calculate_nimber());
        }

        return Self::mex(&mut childNimbers);
    }

    fn mex(nums: &mut Vec<u16>) -> u16{
        nums.sort();
        let mut i: u16 = 0;
        while nums.binary_search(&i).is_ok() {i+=1;}
        return i;
    }

    ///returns all independent parts of the GeneralizedNimGame
    pub fn get_split(&self) -> Vec<GeneralizedNimGame>{
        
        let mut processedNodes = vec![false; self.nodes as usize];
        let mut parts = vec![];

        for i in 0..self.nodes
        {
            if !processedNodes[i as usize] //if the node is not already processed
            {
                let mut currentNodes = vec![];
                processedNodes[i as usize] = true;
                
                currentNodes.push(i);
                
                let mut newPart = vec![];

                while currentNodes.len() != 0 //while there are nodes to process
                {
                    let currentNode = currentNodes.pop().unwrap();

                    newPart.push(currentNode);
                    for neighbour in &self.neighbours[currentNode as usize]
                    {
                        let neighbour = *neighbour;
                        if !processedNodes[neighbour as usize] && currentNodes.contains(&neighbour)//if the node has not already been processed and it is not already queued
                        {
                            processedNodes[neighbour as usize] = true;
                            currentNodes.push(neighbour);
                        }
                    }
                }
                parts.push(self.keep_nodes(&mut newPart).unwrap());
            }
        }
        return parts;
    }
}