use std::{vec, collections::HashMap};

pub mod display;
pub mod symmetries;
pub mod moves;
pub mod nimber;
pub mod impls;

///A generalized version of any impartial "taking game"
///implements many tools to effitiently find the nimber of any complex taking game
#[derive(Debug,Eq)]
pub struct GeneralizedNimGame{
    groups :Vec<Vec<u16>>,
    /// neighbours[i] stores all nodes neighbouring i in ascending order (deduped)
    neighbours :Vec<Vec<u16>>,
    ///the amount of nodes in groups <=> the biggest index
    nodes :u16
}

//Constructors and basic functions for the GeneralizedNimGame
impl GeneralizedNimGame{
    /// gets all neighbours of a given node
    pub fn get_neighbours(&self, node :u16) -> &Vec<u16>{
        return &self.neighbours[node as usize];
    }
    
    ///creates an empty GeneralizedNimGame
    pub fn empty() -> GeneralizedNimGame{
        return GeneralizedNimGame{
            groups: Vec::new(),
            neighbours: Vec::new(),
            nodes: 0
        }
    }
    
    ///creates and simplifies a GeneralizedNimGame from a vec<vec<u16>>
    pub fn new(vec_of_groups :Vec<Vec<u16>>) -> GeneralizedNimGame{ 
        
        let  groups= &mut vec_of_groups.clone();


        let nodes = Self::flatten_indecies_get_node_count(groups);
        Self::remove_unneccesary_data(groups);
        
        let vec_of_groups = groups.to_vec();
        
        let neighbours = Self::build_neighbours(&vec_of_groups, nodes);

        return GeneralizedNimGame{groups: vec_of_groups,nodes:  nodes,neighbours: neighbours};
    }
    
    fn build_neighbours(vec_of_groups :&Vec<Vec<u16>>, nodes: u16) -> Vec<Vec<u16>>{
    
        let mut neighbours = vec![];
    
        for node in 0..nodes{
            let mut nodes_neighbours = vec![];
            
            for group in vec_of_groups {
                if group.contains(&node){
                    nodes_neighbours.append(&mut group.clone());
                }
            }
            //remove all nodes;
            nodes_neighbours.retain(|val| *val != node);
            nodes_neighbours.sort();
            nodes_neighbours.dedup();
            neighbours.push(nodes_neighbours);
        }
        return neighbours;
    }
          
    fn remove_unneccesary_data(vec_of_groups :&mut Vec<Vec<u16>>){

        for i in 0..vec_of_groups.len()
        {
            vec_of_groups[i].dedup();
            vec_of_groups[i].sort();
        }

        vec_of_groups.sort_by(|a,b|a.len().cmp(&b.len()));

        let mut i = 0;
        'outer: while i+1<vec_of_groups.len() {
            if vec_of_groups[i].len() == 0 {
                vec_of_groups.remove(i);
                continue;
            }

            for j in (i+1)..vec_of_groups.len(){
                
                if Self::contains_all(&vec_of_groups[i], &vec_of_groups[j]) {
                    vec_of_groups.remove(i);
                    continue 'outer;
                }
            }

            i+=1;
        }
    }

    fn contains_all(arr1 : &Vec<u16>, arr2 : &Vec<u16>) -> bool{
        
        let mut index1 = 0;
        let mut index2 = 0;
        while index1<arr1.len() && index2<arr2.len() {
            
            if arr1[index1] < arr2[index2]{
                break;
            }

            if arr1[index1] == arr2[index2]{
                index1+=1;
                index2+=1;
            }
            else{
                index2+=1;
            }
        }
        let result = index1 == arr1.len();

        return result;
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


