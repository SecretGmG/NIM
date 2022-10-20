use super::GeneralizedNimGame;
use bitvec::*;

impl GeneralizedNimGame{

    pub fn calculate_nimber(&self) -> u16{
        let mut total_nimber = 0;

        let parts = self.get_split();

        for part in parts{     
            
            let part_nimber = part.get_nimber_with_mex();

            total_nimber ^= part_nimber;
        }
        return total_nimber;
    }
    fn get_nimber_with_mex(&self) -> u16{
        if self.groups.len() == 0 {return 0;}
        if self.groups.len() == 1 {return self.nodes as u16}
        if self.is_symmetric() {return 0;}

        let mut childNimbers = vec![];

        let unique_child_games = self.get_unique_child_games();

        for child in unique_child_games {
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
        
        let mut processedNodes = vec![];
        let mut parts = vec![];

        for i in 0..self.nodes
        {
            if !processedNodes.contains(&i) //if the node is not already processed
            {
                let mut currentNodes = vec![];
                
                processedNodes.push(i); //push i to the processed nodes
                currentNodes.push(i); //push i to the nodes to process
                
                for neighbour in &self.neighbours[i as usize]{
                    currentNodes.push(*neighbour); //push neighbours of i to the nodes to process
                }
                
                let mut newPart = vec![];

                while currentNodes.len() != 0 //while there are nodes to process
                {
                    let currentNode = currentNodes.pop().unwrap();

                    newPart.push(currentNode);

                    for neighbour in &self.neighbours[currentNode as usize]
                    {
                        let neighbour = *neighbour;
                        if !processedNodes.contains(&neighbour){ //if the node has not already been processed
                            
                            processedNodes.push(neighbour);
                            currentNodes.push(neighbour);
                        }
                    }
                }
                if newPart.len() != 0{
                    parts.push(self.keep_nodes(&mut newPart).unwrap());
                }
            }
        }
        return parts;
    }
}