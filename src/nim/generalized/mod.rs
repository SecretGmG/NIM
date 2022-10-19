use std::{vec, collections::HashMap, fs::remove_dir};

pub mod symmetries;
pub mod moves;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game
#[derive(Debug)]
pub struct GeneralizedNimGame{
    groups :Vec<Vec<u16>>,
    /// neighbours[i] stores all nodes neighbouring i in ascending order (deduped)
    neighbours :Option<Vec<Vec<u16>>>,
    ///the amount of nodes in groups <=> the biggest index
    nodes :u16
}

//Constructors and basic functions for the GeneralizedNimGame
impl GeneralizedNimGame{
    /// gets all neighbours of a given node
    pub fn get_neighbours(&mut self, node :u16) -> Vec<u16>{

        match &mut self.neighbours {
            Some(neighbours) => { neighbours[node as usize] },
            None => { self.build_neighbours(); self.neighbours.unwrap()[node as usize] },
            }
    }

    fn build_neighbours(&mut self){

        self.neighbours = Some(vec![]);

        for node in 0..self.nodes{
            let mut nodes_neighbours = vec![];
            
            for group in &self.groups {
                if group.contains(&node){
                    nodes_neighbours.append(&mut group.clone());
                }
            }
            //remove all nodes;
            nodes_neighbours.retain(|val| *val != node);

            nodes_neighbours.sort();
            nodes_neighbours.dedup();

            self.neighbours.unwrap().push(nodes_neighbours);
        }
    }


    ///creates an empty GeneralizedNimGame
    pub fn empty() -> GeneralizedNimGame{
        return GeneralizedNimGame{
            groups: Vec::new(),
            neighbours: None,
            nodes: 0
        }
    }
    
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(vec_of_groups :Vec<Vec<u16>>) -> GeneralizedNimGame{ 

        let  groups= &mut vec_of_groups.clone();


        let nodes = Self::flatten_indecies_get_node_count(groups);
        Self::remove_unneccesary_data(groups);
        

        return GeneralizedNimGame{groups: groups.to_vec(), nodes, neighbours: None};
    }

    fn remove_unneccesary_data(vec_of_groups :&mut Vec<Vec<u16>>){

        let mut unnecessary_groups = vec![];

        for i in 0..vec_of_groups.len()
        {
            vec_of_groups[i].dedup();
            for j in 0..vec_of_groups.len()
            {
                if i != j && i <= j && vec_of_groups[i].iter().all(|e| vec_of_groups[j].contains(e))
                {
                    unnecessary_groups.push(i);
                    break;
                }
            }
        }
        
        //I can simply pop because the unnecessary_groups are sorted in ascending order
        while unnecessary_groups.len() != 0{
            vec_of_groups.remove(unnecessary_groups.pop().unwrap());
        }
    }
    

    fn flatten_indecies_get_node_count(vec_of_groups :&mut Vec<Vec<u16>>) -> u16{
        
        let mut indecies = HashMap::new();
        let mut index: u16 = 0;

        //generate map of new indecies
        for i in 0..vec_of_groups.len(){
            for j in 0..vec_of_groups[i].len(){
                if !indecies.contains_key(&vec_of_groups[i][j]) {
                    indecies.insert(vec_of_groups[i][j], index);
                    index += 1;
                }
            }
        }


        //assign new indecies
        for i in 0..vec_of_groups.len(){
            for j in 0..vec_of_groups[i].len(){
                vec_of_groups[i][j] = *indecies.get(&vec_of_groups[i][j]).unwrap();
            }
        }
        
        return indecies.len() as u16;
    }

    
        //Reasigns indecies in a way that doesn't alter the data to standartisize the format
        fn assign_indecies(vec_of_groups :&mut Vec<Vec<u16>>)
        {
            /*
            //Count Occurences
            for (int i = 0; i < comparer.Length; i++)
            {
            }
            //Sum up All neighbours occurences
            for (int i = 0; i < comparer.Length; i++)
            {
            }
            //Repeat once
            for (int i = 0; i < comparer.Length; i++)
            {
            }

            //Sort a refrence Array By the comparer with ListComparer
            int[] refrences = ArrayCreator.GetIndexed(0,comparer.Length);
            Array.Sort(comparer, refrences, new ListComparer());
            refrences = Inverse(refrences);

            //Reasign Indecies
            for (int i = 0; i < newValues.Count; i++)
            {
                for (int j = 0; j < newValues[i].Count; j++)
                {
                    newValues[i][j] = refrences[newValues[i][j]];
                }
                newValues[i].Sort();
            }

            newValues.Sort(new ListComparer());
            Values = Array.ConvertAll(newValues.ToArray(), (v) => v.ToArray());
        }*/
        todo!();
    }
}


